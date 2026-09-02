// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(delegate = crate::settings_get::settings_get, tag = "admin_settings")]
pub(crate) async fn api_settings(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSettingsError,
> {
}
