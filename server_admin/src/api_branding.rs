// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::settings_branding::settings_branding, tag = "admin_settings")]
pub(crate) async fn api_branding(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminBrandingError,
> {
}
