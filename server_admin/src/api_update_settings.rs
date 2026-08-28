// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(
    delegate = crate::settings_update::settings_update,
    tag = "admin_settings"
)]
pub(crate) async fn api_update_settings(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateSettingsReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminUpdateSettingsError> {
}
