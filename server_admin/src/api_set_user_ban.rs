// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::mutations_set_ban::mutations_set_ban, tag = "admin_users")]
pub(crate) async fn api_set_user_ban(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminSetUserBanReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminSetUserBanError> {
}
