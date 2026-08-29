#![allow(
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branch requires attribute traits after macro expansion"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_data_tables(
    table: Option<&server_admin_contract::admin_data_table_view::AdminDataTableView>,
    query: &server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let content_view = leptos::view! {
        {table.map(|view| leptos::view! {
            <section class="table-page">
                {crate::data_table_grid::data_table_grid(view, query)}
                {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Tables, query.page(), view.total(), Some(view.table()), bool::from(view.table().supports_filters()).then_some(query.filter()))}
            </section>
        })}
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_table_access::render_admin_page_with_table_access(
        server_admin_contract::admin_page::AdminPage::Tables,
        content,
        Some(admin),
        Some(branding),
        table.map(server_admin_contract::admin_data_table_view::AdminDataTableView::table),
    )
}
