pub(crate) async fn sessions_revoke_session(
    auth: crate::AdminAuthReq,
    session: crate::AdminSessionPath,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
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
        .map_err(crate::AdminError::from)?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    crate::repository::revoke_access_session::revoke_access_session(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        session.0,
        authenticated.id,
    )
    .await
    .map_err(crate::AdminError::from)?;
    crate::persistence::record_audit_success_in_connection(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        crate::persistence::AdminAuditSuccessRef {
            action: crate::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: crate::AdminAuditResource::Session,
            resource_id: crate::persistence::AdminAuditResourceId::Session(session.0),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(crate::AdminError::from)?;
    Ok(crate::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
