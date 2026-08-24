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
    page: &server_admin_contract::AdminPermissionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let rows = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let permission = item.name().to_string();
        leptos::view! {
            <crate::ui::table::TableRow><crate::ui::table::TableCell data_label="id">{id}</crate::ui::table::TableCell><crate::ui::table::TableCell data_label="permission">{permission}</crate::ui::table::TableCell></crate::ui::table::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::ui::table::TableWrapper><crate::ui::table::Table><crate::ui::table::TableHeader><crate::ui::table::TableRow><crate::ui::table::TableHead>"id"</crate::ui::table::TableHead><crate::ui::table::TableHead>"permission"</crate::ui::table::TableHead></crate::ui::table::TableRow></crate::ui::table::TableHeader>
        <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody></crate::ui::table::Table></crate::ui::table::TableWrapper>
        {super::table_pagination(server_admin_contract::AdminPage::Permissions, query, page.total(), None, None)}
        </section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Permissions,
        content,
        Some(admin),
        Some(branding),
    )
}
