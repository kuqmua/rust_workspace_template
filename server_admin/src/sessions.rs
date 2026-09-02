#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn sessions(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        *admin_auth_request.get_peer(),
    )
    .await?;
    let total = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_ACTIVE_SESSIONS_SQL)
        .bind(authenticated.get_id().get())
        .fetch_one(admin_auth_request.get_state().as_ref().get_pool().as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_error::AdminError::from)?;
    let items = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        constants_str::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
    )
    .bind(authenticated.get_id().get())
    .bind(i64::from(u16::from(axum_admin_query.get_inner().limit())))
    .bind(i64::from(u32::from(axum_admin_query.get_inner().offset())))
    .fetch_all(admin_auth_request.get_state().as_ref().get_pool().as_ref())
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_error::AdminError::from)?
    .into_iter()
    .map(|(id, created_at, expires_at)| {
        Ok(
            server_admin_contract::admin_session_view::AdminSessionView::new(
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(
                    created_at,
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
                server_admin_contract::admin_session_timestamp::AdminSessionTimestamp::try_from(
                    expires_at,
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
                server_admin_contract::admin_session_identifier::AdminSessionIdentifier::try_from(
                    id.to_string(),
                )
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?,
                server_admin_contract::admin_bool::AdminBool::from(
                    id == authenticated.get_session_id().get().get(),
                ),
            ),
        )
    })
    .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()
    .map_err(crate::map_repository_error::map_repository_error)?;
    let page = server_admin_contract::admin_sessions_page::AdminSessionsPage::new(
        server_admin_contract::admin_session_views::AdminSessionViews::try_from(items)
            .map_err(|_error| {
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
            })
            .map_err(crate::map_repository_error::map_repository_error)?,
        crate::repository_page_total::repository_page_total(
            crate::admin_page_total_count::AdminPageTotalCount::from(total),
        )
        .map_err(crate::map_repository_error::map_repository_error)?,
    );
    Ok(crate::json_response::json_response(page))
}
