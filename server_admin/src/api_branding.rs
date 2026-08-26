#[allow(clippy::single_call_fn)] // Axum route endpoint is registered once by the route inventory
#[frontend_contract::domain_types::route_openapi(delegate = super::settings_branding::branding, tag = "admin_settings")]
pub(super) async fn branding(
    auth: super::AdminAuthReq,
) -> Result<super::AxumAdminResponse, super::AdminBrandingError> {
}
