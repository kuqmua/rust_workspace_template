#![allow(clippy::single_call_fn)] // authenticated document facade has one page-shell caller

pub(super) fn page_render_with_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: super::super::AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
) -> super::super::AdminSsrHtml {
    super::page_render_with_table_access::page_render_with_table_access(
        page, content, admin, branding, None,
    )
}
