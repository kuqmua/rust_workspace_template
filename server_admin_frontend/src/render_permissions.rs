#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub(super) fn render_permissions(
    page: &server_admin_contract::domain_types::AdminPermissionsPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let rows = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let permission = item.name().to_string();
        leptos::view! {
            <crate::domain_types::with_owner::table::table_row::TableRow><crate::domain_types::with_owner::table::table_cell::TableCell data_label="id">{id}</crate::domain_types::with_owner::table::table_cell::TableCell><crate::domain_types::with_owner::table::table_cell::TableCell data_label="permission">{permission}</crate::domain_types::with_owner::table::table_cell::TableCell></crate::domain_types::with_owner::table::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::domain_types::with_owner::table::table_wrapper::TableWrapper><crate::domain_types::with_owner::table::table_impl::Table><crate::domain_types::with_owner::table::table_header::TableHeader><crate::domain_types::with_owner::table::table_row::TableRow><crate::domain_types::with_owner::table::table_head::TableHead>"id"</crate::domain_types::with_owner::table::table_head::TableHead><crate::domain_types::with_owner::table::table_head::TableHead>"permission"</crate::domain_types::with_owner::table::table_head::TableHead></crate::domain_types::with_owner::table::table_row::TableRow></crate::domain_types::with_owner::table::table_header::TableHeader>
        <crate::domain_types::with_owner::table::table_body::TableBody>{rows}</crate::domain_types::with_owner::table::table_body::TableBody></crate::domain_types::with_owner::table::table_impl::Table></crate::domain_types::with_owner::table::table_wrapper::TableWrapper>
        {super::table_pagination(server_admin_contract::domain_types::AdminPage::Permissions, query, page.total(), None, None)}
        </section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::domain_types::AdminPage::Permissions,
        content,
        Some(admin),
        Some(branding),
    )
}
