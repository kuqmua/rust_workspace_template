pub(crate) async fn sessions_revoke_session(
    auth: crate::admin_auth_req::AdminAuthReq,
    session: crate::admin_session_path::AdminSessionPath,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        auth.state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        auth.state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    crate::revoke_access_session::revoke_access_session(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        session.0,
        authenticated.id,
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef {
            action: crate::admin_audit_action::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: crate::admin_audit_resource::AdminAuditResource::Session,
            resource_id: crate::admin_audit_resource_id::AdminAuditResourceId::Session(session.0),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
