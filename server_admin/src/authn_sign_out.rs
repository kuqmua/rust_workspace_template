pub(crate) async fn authn_sign_out(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let peer = auth.peer;
    let state = auth.state;
    let headers = auth.headers;
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    )
    .await?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::AdminError::from)?;
    crate::repository::revoke_access_session::revoke_access_session(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        authenticated.session_id,
        authenticated.id,
    )
    .await
    .map_err(crate::AdminError::from)?;
    if let Some(raw_refresh) = crate::find_admin_cookie(
        crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
        crate::AdminCookieKind::Refresh,
    ) {
        let refresh = crate::SecrecyAdminString::try_from(raw_refresh.as_ref().to_owned())
            .map(crate::AdminOpaqueToken::new)
            .map_err(crate::AdminSecretTextError::from)
            .map_err(crate::AdminError::authentication_secret_text)?;
        let context_hash =
            crate::authorization_session_context_hash::authorization_session_context_hash(
                crate::HttpAdminHeaderMapRef::from(headers.as_ref()),
                peer,
            )
            .map_err(crate::AdminError::authentication_secret_text)?;
        let refresh_hash =
            crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
                &refresh,
                &context_hash,
            )
            .map_err(crate::AdminError::authentication_secret_text)?;
        crate::repository::revoke_refresh_token::revoke_refresh_token(
            crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            &refresh_hash,
            authenticated.id,
        )
        .await
        .map_err(crate::AdminError::from)?;
    }
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::SignOut,
            login: &authenticated.login,
            resource: crate::AdminAuditResource::Session,
            resource_id: crate::persistence::AdminAuditResourceId::Session(
                authenticated.session_id,
            ),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    let mut response = crate::AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    crate::append_cleared_session_cookies::append_cleared_session_cookies(
        &mut response,
        state.as_ref(),
    )?;
    Ok(response)
}
