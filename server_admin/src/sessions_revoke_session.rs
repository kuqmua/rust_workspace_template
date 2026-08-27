#![allow(clippy::single_call_fn)] // route inventory registers this session operation once

pub(super) async fn sessions_revoke_session(
    auth: super::AdminAuthReq,
    session: super::AdminSessionPath,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let authenticated = super::authorization_authenticate::authorization_authenticate(
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
    super::authorization_validate_csrf::authorization_validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    crate::repository::revoke_access_session::revoke_access_session(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        session.0,
        authenticated.id,
    )
    .await
    .map_err(super::AdminError::from)?;
    super::persistence::record_audit_success_in_connection(
        super::persistence::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::persistence::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: super::super::AdminAuditResource::Session,
            resource_id: super::persistence::AdminAuditResourceId::Session(session.0),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
