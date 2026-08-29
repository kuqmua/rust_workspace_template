// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(delegate = crate::settings_get::settings_get, tag = "admin_settings")]
pub(crate) async fn api_settings(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminSettingsError,
> {
}
