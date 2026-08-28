// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::role_mutations_create::role_mutations_create, tag = "admin_roles")]
pub(crate) async fn api_create_role(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminCreateRoleReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminCreateRoleError> {
}
