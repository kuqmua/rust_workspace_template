#[must_use]
pub fn render_sign_in(
    error: Option<crate::AdminSsrErrorMessage>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> crate::AdminSsrHtml {
    super::render::render(error, branding)
}
