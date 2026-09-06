#![allow(
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branch requires attribute traits after macro expansion"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_data_tables(
    option: Option<&server_admin_contract::admin_data_table_view::AdminDataTableView>,
    admin_data_table_query: &server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let content_view = leptos::view! {
        {option.map(|view| leptos::view! {
            <section class="table-page">
                {crate::data_table_grid::data_table_grid(view, admin_data_table_query)}
                {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Tables, admin_data_table_query.page(), view.total(), Some(view.table()), bool::from(view.table().supports_filters()).then_some(admin_data_table_query.filter()))}
            </section>
        })}
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_table_access::render_admin_page_with_table_access(
        server_admin_contract::admin_page::AdminPage::Tables,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
        option.map(server_admin_contract::admin_data_table_view::AdminDataTableView::table),
    )
}
