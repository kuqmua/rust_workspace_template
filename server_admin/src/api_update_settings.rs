// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(
    delegate = crate::settings_update::settings_update,
    tag = "admin_settings"
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
}
