#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused session operations once

pub(super) async fn sessions_view(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminSessionsPage, super::AdminError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    super::super::repository::sessions::list_active_sessions(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        authenticated.session_id,
        authenticated.id,
        &query.0,
    )
    .await
    .map_err(super::shared::map_repository_error)
}
pub(super) async fn sessions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    sessions_view(auth, query)
        .await
        .map(super::shared::json_response)
}
pub(super) async fn revoke_session(
    auth: super::AdminAuthReq,
    session: super::AdminSessionPath,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
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
        .map_err(super::AdminError::from)?;
    super::validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    super::super::repository::sessions::revoke_access_session(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        session.0,
        authenticated.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::AdminAuditResourceId::Session(session.0),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn revoke_all_sessions(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    super::validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
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
        .map_err(super::AdminError::from)?;
    super::super::repository::sessions::revoke_user_sessions(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        authenticated.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::AdminAuditResourceId::Session(authenticated.session_id),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    let mut response = super::AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    super::append_cleared_session_cookies(&mut response, auth.state.as_ref())?;
    Ok(response)
}
