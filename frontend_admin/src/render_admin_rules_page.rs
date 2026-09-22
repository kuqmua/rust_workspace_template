#![allow(
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[must_use]
pub fn render_admin_rules_page(
    admin_rules_page: &server_admin_contract::admin_rules_page::AdminRulesPage,
    admin_table_query: &server_admin_contract::admin_table_query::AdminTableQuery,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let rows = admin_rules_page.items().iter().map(|item| {
        let id = item.id().to_string();
        let name = item.name().to_string();
        let created_at = item.created_at().to_string();
        leptos::view! {
            <crate::table_row::TableRow><crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell><crate::table_cell::TableCell data_label="name">{name}</crate::table_cell::TableCell><crate::table_cell::TableCell data_label=constants_str::CREATED_AT>{created_at}</crate::table_cell::TableCell><crate::table_cell::TableCell data_label=constants_str::ADMIN_UI_ACTIONS bool=true>{constants_str::EMPTY}</crate::table_cell::TableCell></crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"name"</crate::table_head::TableHead><crate::table_head::TableHead>{constants_str::CREATED_AT}</crate::table_head::TableHead><crate::table_head::TableHead>{constants_str::ADMIN_UI_ACTIONS}</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
        <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
        {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Rules, admin_table_query, admin_rules_page.total(), None, None)}
        </section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Rules,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
    )
}
