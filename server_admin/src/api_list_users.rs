// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(
    delegate = crate::user_queries_list::user_queries_list,
    params(server_admin_contract::admin_table_query::AdminTableQuery),
    tag = "admin_users"
)]
pub(crate) async fn api_list_users(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminListUsersError,
> {
}
