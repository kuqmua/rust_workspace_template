#[proc_macro_frontend_contract::route_openapi(tag = "admin_settings")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_settings(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_settings_request::AdminUpdateSettingsRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUpdateSettingsError,
> {
    crate::settings_update::settings_update(admin_auth_request, axum_admin_json)
        .await
        .map_err(crate::application_auth::AdminUpdateSettingsError::from)
}
