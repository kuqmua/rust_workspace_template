#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::authn_refresh::authn_refresh, tag = "admin_auth")]
pub(super) async fn api_refresh(
    auth: super::AdminAuthReq,
    peer: super::AdminPeerAddr,
) -> Result<super::AxumAdminResponse, super::AdminRefreshError> {
}
