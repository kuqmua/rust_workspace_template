// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::authn_refresh::authn_refresh, tag = "admin_auth")]
pub(crate) async fn api_refresh(
    auth: crate::AdminAuthReq,
    peer: crate::AdminPeerAddr,
) -> Result<crate::AxumAdminResponse, crate::AdminRefreshError> {
}
