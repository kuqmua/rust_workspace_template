// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::authn_sign_in::authn_sign_in, tag = "admin_auth")]
pub(crate) async fn api_sign_in(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
    admin_sign_in_json: crate::admin_sign_in_json::AdminSignInJson,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::application_auth::AdminSignInError>
{
}
