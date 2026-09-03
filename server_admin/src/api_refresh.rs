#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_auth")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_refresh(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::application_auth::AdminRefreshError>
{
    crate::authn_refresh::authn_refresh(admin_auth_request, admin_peer_addr)
        .await
        .map_err(crate::application_auth::AdminRefreshError::from)
}
