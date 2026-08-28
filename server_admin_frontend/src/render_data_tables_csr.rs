#[must_use]
pub fn render_data_tables_csr(
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> crate::domain_types::ssr::AdminSsrHtml {
    super::render_admin_csr::render_admin_csr(
        server_admin_contract::domain_types::AdminPage::Tables,
        active_table,
        admin,
        branding,
    )
}
