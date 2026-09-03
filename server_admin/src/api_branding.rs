#[proc_macro_frontend_contract::route_openapi(tag = "admin_settings")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_branding(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminBrandingError,
> {
    crate::settings_branding::settings_branding(admin_auth_request)
        .await
        .map_err(crate::application_auth::AdminBrandingError::from)
}
