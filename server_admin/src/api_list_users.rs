// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::users::user_queries_list::user_queries_list,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_users"
)]
pub(super) async fn api_list_users(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminListUsersError> {
}
