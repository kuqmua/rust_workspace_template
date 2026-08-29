// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(delegate = crate::mutations_set_ban::mutations_set_ban, tag = "admin_users")]
pub(crate) async fn api_set_user_ban(
    auth: crate::admin_auth_req::AdminAuthReq,
    path: crate::axum_admin_path::AxumAdminPath<server_admin_core::admin_user_id::AdminUserId>,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_set_user_ban_req::AdminSetUserBanReq,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSetUserBanError,
> {
}
