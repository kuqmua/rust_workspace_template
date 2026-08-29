#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub fn render_admin_permissions_page(
    page: &server_admin_contract::admin_permissions_page::AdminPermissionsPage,
    query: &server_admin_contract::admin_table_query::AdminTableQuery,
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let rows = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let permission = item.name().to_string();
        leptos::view! {
            <crate::table_row::TableRow><crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell><crate::table_cell::TableCell data_label="permission">{permission}</crate::table_cell::TableCell></crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"id"</crate::table_head::TableHead><crate::table_head::TableHead>"permission"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
        <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
        {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Permissions, query, page.total(), None, None)}
        </section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Permissions,
        content,
        Some(admin),
        Some(branding),
    )
}
