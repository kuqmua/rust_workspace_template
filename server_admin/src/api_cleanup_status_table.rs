#[proc_macro_frontend_contract_route_openapi::route_openapi(
    params(server_admin_contract::admin_data_table_query::AdminDataTableQuery),
    tag = "admin_tables"
)]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_cleanup_status_table(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminCleanupStatusTableError,
> {
    crate::data_tables_get::data_tables_get(
        admin_auth_request,
        server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus,
        axum_admin_query,
    )
    .await
    .map_err(crate::application_auth::AdminCleanupStatusTableError::from)
}
