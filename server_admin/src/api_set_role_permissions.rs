// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(
    delegate = crate::mutations_set_permissions::mutations_set_permissions,
    tag = "admin_roles"
)]
pub(crate) async fn api_set_role_permissions(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminRoleId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminSetRolePermissionsReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminSetRolePermissionsError> {
}
