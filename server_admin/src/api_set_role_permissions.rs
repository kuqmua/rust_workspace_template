#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::roles::mutations_set_permissions::mutations_set_permissions,
    tag = "admin_roles"
)]
pub(super) async fn api_set_role_permissions(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetRolePermissionsReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetRolePermissionsError> {
}
