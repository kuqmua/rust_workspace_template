use super::{AdminSsrHtml, crud_render_role_manage};

#[must_use]
pub fn render_role_manage(
    page: &server_admin_contract::domain_types::AdminRolesPage,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    crud_render_role_manage(page, admin, branding)
}
