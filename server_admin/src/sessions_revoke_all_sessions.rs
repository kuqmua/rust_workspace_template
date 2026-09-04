#[allow(
    clippy::single_call_fn,
    reason = "sessions revoke all sessions remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn sessions_revoke_all_sessions(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        *admin_auth_request.get_peer(),
    )
    .await?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        &authenticated,
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
    crate::revoke_user_sessions::revoke_user_sessions(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
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
                *authenticated.get_session_id(),
            ),
            *authenticated.get_id(),
        ),
    )
    .await?;
    let mut response = crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    );
    crate::append_cleared_session_cookies::append_cleared_session_cookies(
        &mut response,
        admin_auth_request.get_state().as_ref(),
    )?;
    Ok(response)
}
