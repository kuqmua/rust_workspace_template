#![allow(clippy::single_call_fn)] // HTML composition owns the branding view operation

pub(super) async fn branding_view(
    auth: super::AdminAuthReq,
) -> Result<server_admin_contract::domain_types::AdminBrandingView, super::AdminError> {
    super::settings_branding_view_ref::branding_view_ref(&auth).await
}
