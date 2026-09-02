// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::authn_refresh::authn_refresh, tag = "admin_auth")]
pub(crate) async fn api_refresh(
    auth: crate::admin_auth_req::AdminAuthReq,
    peer: crate::admin_peer_addr::AdminPeerAddr,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::application_auth::AdminRefreshError>
{
}
