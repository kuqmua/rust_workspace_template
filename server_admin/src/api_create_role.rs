#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::roles::role_mutations_create::role_mutations_create, tag = "admin_roles")]
pub(super) async fn api_create_role(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminCreateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminCreateRoleError> {
}
