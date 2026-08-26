#![allow(clippy::single_call_fn)] // SSR facade has one table page renderer owner

pub(in crate::domain_types::ssr) fn render_admin_page_with_table_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: super::super::AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> super::super::AdminSsrHtml {
    super::page_render_with_table_access::render_with_table_access(
        page,
        content,
        admin,
        branding,
        active_table,
    )
}
