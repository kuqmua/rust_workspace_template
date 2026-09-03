#[proc_macro_frontend_contract::route_openapi(tag = "admin_auth")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_sign_in(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
    admin_sign_in_json: crate::admin_sign_in_json::AdminSignInJson,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::application_auth::AdminSignInError>
{
    crate::authn_sign_in::authn_sign_in(admin_auth_request, admin_peer_addr, admin_sign_in_json)
        .await
        .map_err(crate::application_auth::AdminSignInError::from)
}
