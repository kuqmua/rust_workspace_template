pub(crate) fn render_admin_page_with_table_access(
    page: server_admin_contract::admin_page::AdminPage,
    content: crate::admin_ssr_html::AdminSsrHtml,
    admin: Option<&server_admin_contract::authenticated_admin::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::admin_branding_view::AdminBrandingView>,
    active_table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::page_render_with_table_access::page_render_with_table_access(
        page,
        content,
        admin,
        branding,
        active_table,
    )
}
