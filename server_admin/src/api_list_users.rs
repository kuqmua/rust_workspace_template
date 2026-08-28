// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(
    delegate = crate::user_queries_list::user_queries_list,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_users"
)]
pub(crate) async fn api_list_users(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminListUsersError> {
}
