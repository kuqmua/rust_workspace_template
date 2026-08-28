// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(delegate = crate::mutations_set_roles::mutations_set_roles, tag = "admin_users")]
pub(crate) async fn api_set_user_roles(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserRolesReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminSetUserRolesError> {
}
