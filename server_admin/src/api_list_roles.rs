// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(
    delegate = crate::role_queries_list::role_queries_list,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_roles"
)]
pub(crate) async fn api_list_roles(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminListRolesError> {
}
