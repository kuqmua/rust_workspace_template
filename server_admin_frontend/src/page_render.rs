#![allow(clippy::single_call_fn)] // document facade has one page-shell caller

pub(super) fn page_render(
    page: server_admin_contract::domain_types::AdminPage,
    content: super::super::AdminSsrHtml,
) -> super::super::AdminSsrHtml {
    super::page_render_with_access::page_render_with_access(page, content, None, None)
}
