#[must_use]
pub(crate) fn render_admin_page(
    page: server_admin_contract::domain_types::AdminPage,
    content: crate::AdminSsrHtml,
) -> crate::AdminSsrHtml {
    super::page_render::page_render(page, content)
}
