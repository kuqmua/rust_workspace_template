#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused session operations once

pub(super) async fn sessions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let authenticated = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    let total = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_ACTIVE_SESSIONS_SQL)
        .bind(authenticated.id.get())
        .fetch_one(auth.state.as_ref().pool.as_ref())
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(super::AdminError::from)?;
    let items = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        constants_str::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
    )
    .bind(authenticated.id.get())
    .bind(i64::from(u16::from(query.0.limit())))
    .bind(i64::from(u32::from(query.0.offset())))
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(crate::domain_types::SqlxAdminError::from)
    .map_err(super::AdminError::from)?
    .into_iter()
    .map(|(id, created_at, expires_at)| {
        Ok(server_admin_contract::domain_types::AdminSessionView::new(
            server_admin_contract::domain_types::AdminSessionTimestamp::try_from(created_at)
                .map_err(|_error| {
                    crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                })?,
            server_admin_contract::domain_types::AdminSessionTimestamp::try_from(expires_at)
                .map_err(|_error| {
                    crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
                })?,
            server_admin_contract::domain_types::AdminSessionIdentifier::try_from(id.to_string())
                .map_err(|_error| {
                crate::adapters::repository::AdminRepositoryError::InvalidStoredValue
            })?,
            server_admin_contract::domain_types::AdminBool::from(
                id == authenticated.session_id.get().get(),
            ),
        ))
    })
    .collect::<Result<Vec<_>, crate::adapters::repository::AdminRepositoryError>>()
    .map_err(super::shared::map_repository_error)?;
    let page = server_admin_contract::domain_types::AdminSessionsPage::new(
        server_admin_contract::domain_types::AdminSessionViews::try_from(items)
            .map_err(|_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue)
            .map_err(super::shared::map_repository_error)?,
        crate::adapters::repository::page_total(
            crate::adapters::repository::AdminPageTotalCount::from(total),
        )
        .map_err(super::shared::map_repository_error)?,
    );
    Ok(super::shared::json_response(page))
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
    crate::adapters::repository::sessions::revoke_access_session(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
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
    crate::adapters::repository::sessions::revoke_user_sessions(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
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
