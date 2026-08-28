// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::settings_branding::settings_branding, tag = "admin_settings")]
pub(crate) async fn api_branding(
    auth: crate::AdminAuthReq,
) -> Result<crate::AxumAdminResponse, crate::AdminBrandingError> {
}
