use super::{AdminSsrHtml, render_settings};

#[must_use]
pub fn render_admin_settings_page(
    view: &server_admin_contract::domain_types::AdminSettingsView,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_settings(view, admin, branding)
}
