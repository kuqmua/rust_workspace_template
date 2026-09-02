#[must_use]
#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn render_admin_page(
    admin_page: server_admin_contract::admin_page::AdminPage,
    admin_ssr_html: crate::admin_ssr_html::AdminSsrHtml,
) -> crate::admin_ssr_html::AdminSsrHtml {
    super::page_render_with_access::page_render_with_access(admin_page, admin_ssr_html, None, None)
}
