#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::account_me::account_me, tag = "admin_auth")]
pub(super) async fn api_me(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminMeError> {
}
