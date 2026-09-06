pub(crate) fn render_admin_page_with_access(
    admin_page: server_admin_contract::admin_page::AdminPage,
    admin_ssr_html: crate::admin_ssr_html::AdminSsrHtml,
    admin: Option<&server_admin_contract::authenticated_admin::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::admin_branding_view::AdminBrandingView>,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::page_render_with_access::page_render_with_access(
        admin_page,
        admin_ssr_html,
        admin,
        branding,
    )
}
