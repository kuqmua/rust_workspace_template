#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::roles::queries_list_permissions::list_permissions,
    params(server_admin_contract::domain_types::AdminTableQuery),
    tag = "admin_roles"
)]
pub(super) async fn list_permissions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::domain_types::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminListPermissionsError> {
}
