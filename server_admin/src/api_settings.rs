#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_settings")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_settings(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSettingsError,
> {
    crate::settings_get::settings_get(admin_auth_request)
        .await
        .map_err(crate::application_auth::AdminSettingsError::from)
}
