#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(super) fn page_render(
    page: server_admin_contract::domain_types::AdminPage,
    content: crate::AdminSsrHtml,
) -> crate::AdminSsrHtml {
    super::page_render_with_access::page_render_with_access(page, content, None, None)
}
