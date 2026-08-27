#![allow(clippy::single_call_fn)] // SSR facade has one sign-in renderer owner

#[must_use]
pub fn render_sign_in(
    error: Option<super::super::AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> super::super::AdminSsrHtml {
    super::render::render(error, branding)
}
