pub(crate) async fn sessions_revoke_session(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_session_path: crate::admin_session_path::AdminSessionPath,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        *admin_auth_request.get_peer(),
    )
    .await?;
    let mut tx = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        &authenticated,
    )
    .await?;
    crate::revoke_access_session::revoke_access_session(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        *admin_session_path.get_inner(),
        *authenticated.get_id(),
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    crate::finalize_audited_transaction::finalize_audited_transaction(
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(tx),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Delete,
            authenticated.get_login(),
            crate::admin_audit_resource::AdminAuditResource::Session,
            crate::admin_audit_resource_id::AdminAuditResourceId::Session(
                *admin_session_path.get_inner(),
            ),
            *authenticated.get_id(),
        ),
    )
    .await?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
