// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::role_mutations_update::role_mutations_update, tag = "admin_roles")]
pub(crate) async fn api_update_role(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminRoleId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateRoleReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminUpdateRoleError> {
}
