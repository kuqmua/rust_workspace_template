use super::{AdminSsrHtml, crud_render_role_create};

#[must_use]
pub fn render_role_create(
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    crud_render_role_create(admin, branding)
}
