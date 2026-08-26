#![allow(clippy::single_call_fn)] // SSR facade has one page renderer owner

#[must_use]
pub(in crate::domain_types::ssr) fn render_admin_page(
    page: server_admin_contract::domain_types::AdminPage,
    content: super::super::AdminSsrHtml,
) -> super::super::AdminSsrHtml {
    super::page_render::page_render(page, content)
}
