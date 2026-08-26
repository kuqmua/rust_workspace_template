#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::authn_sign_out::authn_sign_out, tag = "admin_auth")]
pub(super) async fn api_sign_out(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminSignOutError> {
}
