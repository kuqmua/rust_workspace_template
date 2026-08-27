use super::{AdminSsrHtml, render_sessions};

#[must_use]
pub fn render_admin_sessions_page(
    page: &server_admin_contract::domain_types::AdminSessionsPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_sessions::render_sessions(page, query, admin, branding)
}
