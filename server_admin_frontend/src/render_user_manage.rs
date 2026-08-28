use super::{AdminSsrHtml, crud_render_user_manage};

#[must_use]
pub fn render_user_manage(
    page: &server_admin_contract::domain_types::AdminUsersPage,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> AdminSsrHtml {
    crud_render_user_manage(page, admin, branding)
}
