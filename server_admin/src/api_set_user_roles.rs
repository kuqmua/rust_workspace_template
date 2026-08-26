#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::mutations_set_roles::mutations_set_roles, tag = "admin_users")]
pub(super) async fn api_set_user_roles(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserRolesReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetUserRolesError> {
}
