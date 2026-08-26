#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::users::mutations_set_ban::set_ban, tag = "admin_users")]
pub(super) async fn set_user_ban(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminUserId>,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserBanReq>,
) -> Result<super::AxumAdminResponse, super::AdminSetUserBanError> {
}
