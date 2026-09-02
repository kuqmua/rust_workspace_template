pub(crate) async fn settings_branding_view(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
) -> Result<
    server_admin_contract::admin_branding_view::AdminBrandingView,
    crate::admin_error::AdminError,
> {
    crate::settings_branding_view_ref::settings_branding_view_ref(&admin_auth_request).await
}
