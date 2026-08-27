// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::roles::role_queries_list::role_queries_list,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_roles"
)]
pub(super) async fn api_list_roles(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminListRolesError> {
}
