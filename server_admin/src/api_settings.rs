// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::settings_get::settings_get, tag = "admin_settings")]
pub(crate) async fn api_settings(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminSettingsError> {
}
