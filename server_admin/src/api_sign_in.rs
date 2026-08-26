#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::authn_sign_in::authn_sign_in, tag = "admin_auth")]
pub(super) async fn api_sign_in(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
    request_json: super::AdminSignInJson,
) -> Result<super::AxumAdminResponse, super::AdminSignInError> {
}
