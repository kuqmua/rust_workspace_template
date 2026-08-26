#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(
    delegate = super::settings_update::update,
    tag = "admin_settings"
)]
pub(super) async fn update_settings(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateSettingsReq>,
) -> Result<super::AxumAdminResponse, super::AdminUpdateSettingsError> {
}
