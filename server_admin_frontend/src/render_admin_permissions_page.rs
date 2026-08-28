use super::{AdminSsrHtml, render_permissions};

#[must_use]
pub fn render_admin_permissions_page(
    page: &server_admin_contract::domain_types::AdminPermissionsPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    render_permissions::render_permissions(page, query, admin, branding)
}
