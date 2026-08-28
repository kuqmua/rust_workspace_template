// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::authn_sign_out::authn_sign_out, tag = "admin_auth")]
pub(crate) async fn api_sign_out(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminSignOutError> {
}
