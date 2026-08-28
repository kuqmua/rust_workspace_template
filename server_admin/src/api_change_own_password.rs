// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(
    delegate = crate::account_change_own_password::account_change_own_password,
    tag = "admin_auth"
)]
pub(crate) async fn api_change_own_password(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminChangeOwnPasswordReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminChangeOwnPasswordError> {
}
