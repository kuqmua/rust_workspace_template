#![allow(clippy::single_call_fn)] // route inventory registers this authentication operation once

pub(super) async fn authn_sign_out(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let peer = auth.peer;
    let state = auth.state;
    let headers = auth.headers;
    let authenticated = super::authorization_authenticate::authorization_authenticate(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    )
    .await?;
    super::authorization_validate_csrf::authorization_validate_csrf(
        state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    crate::adapters::repository::revoke_access_session::revoke_access_session(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        authenticated.session_id,
        authenticated.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    if let Some(raw_refresh) = super::super::find_admin_cookie(
        super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        super::super::AdminCookieKind::Refresh,
    ) {
        let refresh = super::super::SecrecyAdminString::try_from(raw_refresh.as_ref().to_owned())
            .map(super::super::AdminOpaqueToken::new)
            .map_err(super::super::AdminSecretTextError::from)
            .map_err(super::AdminError::authentication_secret_text)?;
        let context_hash =
            super::authorization_session_context_hash::authorization_session_context_hash(
                super::super::HttpAdminHeaderMapRef::from(headers.as_ref()),
                peer,
            )
            .map_err(super::AdminError::authentication_secret_text)?;
        let refresh_hash =
            super::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
                &refresh,
                &context_hash,
            )
            .map_err(super::AdminError::authentication_secret_text)?;
        crate::adapters::repository::revoke_refresh_token::revoke_refresh_token(
            crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            &refresh_hash,
            authenticated.id,
        )
        .await
        .map_err(super::AdminError::from)?;
    }
    super::persistence::record_audit_success_in_connection(
        super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::persistence::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::SignOut,
            login: &authenticated.login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::persistence::AdminAuditResourceId::Session(
                authenticated.session_id,
            ),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    let mut response = super::AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    super::append_cleared_session_cookies::append_cleared_session_cookies(
        &mut response,
        state.as_ref(),
    )?;
    Ok(response)
}
