// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::authn_sign_in::authn_sign_in, tag = "admin_auth")]
pub(crate) async fn api_sign_in(
    auth: crate::AdminAuthReq,
    peer: crate::AdminPeerAddr,
    request_json: crate::AdminSignInJson,
) -> Result<crate::AxumAdminResponse, crate::AdminSignInError> {
}
