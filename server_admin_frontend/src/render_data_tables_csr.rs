#[must_use]
pub fn render_data_tables_csr(
    active_table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::render_admin_csr::render_admin_csr(
        server_admin_contract::admin_page::AdminPage::Tables,
        active_table,
        admin,
        branding,
    )
}
