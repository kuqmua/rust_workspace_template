#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sessions_revoke_all_sessions(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        auth.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.get_headers().as_ref()),
        *auth.get_peer(),
    )
    .await?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        auth.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.get_headers().as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = auth
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::revoke_user_sessions::revoke_user_sessions(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        *authenticated.get_id(),
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Delete,
            authenticated.get_login(),
            crate::admin_audit_resource::AdminAuditResource::Session,
            crate::admin_audit_resource_id::AdminAuditResourceId::Session(
                *authenticated.get_session_id(),
            ),
            *authenticated.get_id(),
        ),
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    let mut response = crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    );
    crate::append_cleared_session_cookies::append_cleared_session_cookies(
        &mut response,
        auth.get_state().as_ref(),
    )?;
    Ok(response)
}
