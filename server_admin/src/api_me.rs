// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(delegate = crate::account_me::account_me, tag = "admin_auth")]
pub(crate) async fn api_me(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminMeError> {
}
