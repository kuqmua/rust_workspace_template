// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::authn_sign_out::authn_sign_out, tag = "admin_auth")]
pub(crate) async fn api_sign_out(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::application_auth::AdminSignOutError>
{
}
