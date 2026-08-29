pub(super) fn page_render_with_access(
    page: server_admin_contract::admin_page::AdminPage,
    content: crate::admin_ssr_html::AdminSsrHtml,
    admin: Option<&server_admin_contract::authenticated_admin::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::admin_branding_view::AdminBrandingView>,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::page_render_with_table_access::page_render_with_table_access(
        page, content, admin, branding, None,
    )
}
