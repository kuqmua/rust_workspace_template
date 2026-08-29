// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(
    delegate = crate::data_tables_get::data_tables_get,
    params(server_admin_contract::admin_data_table_query::AdminDataTableQuery),
    tag = "admin_tables"
)]
pub(crate) async fn api_data_table(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_data_table::AdminDataTable,
    >,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminDataTableError,
> {
}
