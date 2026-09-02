#[must_use]
pub fn render_data_tables_csr(
    option: Option<server_admin_contract::admin_data_table::AdminDataTable>,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::render_admin_csr::render_admin_csr(
        server_admin_contract::admin_page::AdminPage::Tables,
        option,
        authenticated_admin,
        admin_branding_view,
    )
}
