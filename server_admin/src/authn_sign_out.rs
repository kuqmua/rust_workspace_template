pub(crate) async fn authn_sign_out(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let peer = auth.peer;
    let state = auth.state;
    let headers = auth.headers;
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
        peer,
    )
    .await?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::revoke_access_session::revoke_access_session(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        authenticated.session_id,
        authenticated.id,
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    if let Some(raw_refresh) = crate::find_admin_cookie::find_admin_cookie(
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
        crate::admin_cookie_kind::AdminCookieKind::Refresh,
    ) {
        let refresh = server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
            raw_refresh.as_ref().to_owned(),
        )
        .map(crate::admin_opaque_token::AdminOpaqueToken::new)
        .map_err(crate::admin_secret_text_error::AdminSecretTextError::from)
        .map_err(crate::admin_error::AdminError::authentication_secret_text)?;
        let context_hash =
            crate::authorization_session_context_hash::authorization_session_context_hash(
                crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(headers.as_ref()),
                peer,
            )
            .map_err(crate::admin_error::AdminError::authentication_secret_text)?;
        let refresh_hash =
            crate::authorization_hash_refresh_token_with_context::authorization_hash_refresh_token_with_context(
                &refresh,
                &context_hash,
            )
            .map_err(crate::admin_error::AdminError::authentication_secret_text)?;
        crate::revoke_refresh_token::revoke_refresh_token(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
            &refresh_hash,
            authenticated.id,
        )
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    }
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::SignOut,
            login: &authenticated.login,
            resource: crate::admin_audit_resource::AdminAuditResource::Session,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::Session(
                authenticated.session_id,
            ),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let mut response = crate::axum_admin_response::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    );
    crate::append_cleared_session_cookies::append_cleared_session_cookies(
        &mut response,
        state.as_ref(),
    )?;
    Ok(response)
}
