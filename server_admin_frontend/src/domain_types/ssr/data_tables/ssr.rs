#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branch requires attribute traits after macro expansion"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub(in crate::domain_types::ssr) fn render_data_tables(
    table: Option<&server_admin_contract::domain_types::AdminDataTableView>,
    query: &server_admin_contract::domain_types::AdminDataTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> crate::domain_types::ssr::AdminSsrHtml {
    let content_view = leptos::view! {
        {table.map(|view| leptos::view! {
            <section class="table-page">
                {crate::domain_types::ssr::data_table_grid(view, query)}
                {crate::domain_types::ssr::table_pagination(server_admin_contract::domain_types::AdminPage::Tables, query.page(), view.total(), Some(view.table()), bool::from(view.table().supports_filters()).then_some(query.filter()))}
            </section>
        })}
    };
    let content = crate::domain_types::ssr::render_view(content_view);
    crate::domain_types::ssr::render_admin_page_with_table_access(
        server_admin_contract::domain_types::AdminPage::Tables,
        content,
        Some(admin),
        Some(branding),
        table.map(server_admin_contract::domain_types::AdminDataTableView::table),
    )
}
