pub(super) async fn page_context_impl(
    auth: &super::super::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminBrandingView,
        super::super::super::AdminPasswordChangeRequired,
    ),
    super::super::AdminError,
> {
    let (admin, password_change_required) =
        super::super::account_me_context_view_ref::account_me_context_view_ref(auth).await?;
    let branding =
        super::super::settings_branding_view_ref::settings_branding_view_ref(auth).await?;
    Ok((admin, branding, password_change_required))
}
