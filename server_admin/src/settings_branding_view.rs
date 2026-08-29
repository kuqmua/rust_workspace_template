pub(crate) async fn settings_branding_view(
    auth: crate::admin_auth_req::AdminAuthReq,
) -> Result<
    server_admin_contract::admin_branding_view::AdminBrandingView,
    crate::admin_error::AdminError,
> {
    crate::settings_branding_view_ref::settings_branding_view_ref(&auth).await
}
