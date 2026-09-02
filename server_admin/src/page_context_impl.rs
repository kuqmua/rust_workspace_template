pub(crate) async fn page_context_impl(
    admin_auth_request: &crate::admin_auth_request::AdminAuthRequest,
) -> Result<
    (
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        server_admin_contract::admin_branding_view::AdminBrandingView,
        crate::admin_password_change_required::AdminPasswordChangeRequired,
    ),
    crate::admin_error::AdminError,
> {
    let (admin, password_change_required) =
        crate::account_me_context_view_ref::account_me_context_view_ref(admin_auth_request).await?;
    let branding =
        crate::settings_branding_view_ref::settings_branding_view_ref(admin_auth_request).await?;
    Ok((admin, branding, password_change_required))
}
