#![allow(clippy::single_call_fn)] // route inventory and HTML composition each register focused session operations once

pub(super) async fn sessions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let authenticated = super::authorization_authenticate::authorization_authenticate(
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
    .map_err(super::shared::map_repository_error::map_repository_error)?;
    let page = server_admin_contract::domain_types::AdminSessionsPage::new(
        server_admin_contract::domain_types::AdminSessionViews::try_from(items)
            .map_err(|_error| crate::adapters::repository::AdminRepositoryError::InvalidStoredValue)
            .map_err(super::shared::map_repository_error::map_repository_error)?,
        crate::adapters::repository::page_total(
            crate::adapters::repository::AdminPageTotalCount::from(total),
        )
        .map_err(super::shared::map_repository_error::map_repository_error)?,
    );
    Ok(super::shared::json_response::json_response(page))
}
