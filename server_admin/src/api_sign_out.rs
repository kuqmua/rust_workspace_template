#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_auth")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_sign_out(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::application_auth::AdminSignOutError>
{
    crate::authn_sign_out::authn_sign_out(admin_auth_request)
        .await
        .map_err(crate::application_auth::AdminSignOutError::from)
}
