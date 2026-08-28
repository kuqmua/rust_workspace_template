#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sessions_revoke_all_sessions(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
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
    crate::repository::revoke_user_sessions::revoke_user_sessions(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
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
        auth.state.as_ref(),
    )?;
    Ok(response)
}
