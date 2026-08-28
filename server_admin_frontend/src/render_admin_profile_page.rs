use super::{AdminSsrHtml, render_profile};

#[must_use]
pub fn render_admin_profile_page(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_profile(admin, branding)
}
