pub(crate) async fn settings_branding_view(
    auth: crate::AdminAuthReq,
) -> Result<server_admin_contract::domain_types::AdminBrandingView, crate::AdminError> {
    crate::settings_branding_view_ref::settings_branding_view_ref(&auth).await
}
