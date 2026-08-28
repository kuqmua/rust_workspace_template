pub(crate) fn render_admin_page_with_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: crate::AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> crate::AdminSsrHtml {
    super::page_render_with_access::page_render_with_access(page, content, admin, branding)
}
