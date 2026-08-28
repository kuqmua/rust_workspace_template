#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the administrator page shell requires its local set of document attribute traits"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

pub(super) fn page_render_with_table_access(
    page: server_admin_contract::domain_types::AdminPage,
    content: crate::AdminSsrHtml,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    branding: Option<&server_admin_contract::domain_types::AdminBrandingView>,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> crate::AdminSsrHtml {
    let spec = page.spec();
    let title = spec.title();
    let document_title = branding
        .and_then(server_admin_contract::domain_types::AdminBrandingView::tab_title)
        .map_or_else(
            || title.as_ref().to_owned(),
            |value| AsRef::<str>::as_ref(value).to_owned(),
        );
    let primary_color = branding
        .and_then(server_admin_contract::domain_types::AdminBrandingView::primary_color)
        .map(|value| format!("--accent:{}", AsRef::<str>::as_ref(value)));
    let navigation = {
        let tables = server_admin_contract::domain_types::AdminDataTable::PG_ORDER
            .into_iter()
            .filter(|table| {
                admin.is_none_or(|value| {
                    bool::from(value.has_permission(
                        server_admin_contract::domain_types::AdminPermission::TablesRead,
                    )) && bool::from(value.has_permission(table.permission()))
                })
            })
            .map(|table| {
                let name = table.to_string();
                let href = table.frontend_path().to_string();
                leptos::view! {
                    <crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem><crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink active=active_table == Some(table) href=href>{name}</crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink></crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem>
                }
            })
            .collect::<Vec<_>>();
        let pages = server_admin_contract::domain_types::AdminPage::navigation()
            .filter(|item_page| {
                admin.is_none_or(|value| bool::from(value.can_access(*item_page)))
            })
            .map(|item_page| {
                let item = item_page.spec();
                let href = String::from(item.path());
                let label = item.route_name().as_ref().to_owned();
                leptos::view! {
                    <crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem><crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink active=item_page == page href=href>{label}</crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink></crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem>
                }
            })
            .collect::<Vec<_>>();
        leptos::view! {
            <crate::domain_types::with_owner::navigation::admin_sidebar::AdminSidebar>
                {tables}
                {pages}
                <crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem><form method="post" action=server_admin_contract::domain_types::AdminHtmlAction::SignOut.get()><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>{server_admin_contract::domain_types::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</crate::domain_types::with_owner::button::AdminButton></form></crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem>
            </crate::domain_types::with_owner::navigation::admin_sidebar::AdminSidebar>
        }
    };
    super::render_document::render_document(
        &crate::AdminSsrText::try_from(document_title).unwrap_or_else(crate::AdminSsrText::from),
        leptos::view! {
            <div class="app-shell" style=primary_color>
                <header class="topbar">
                    {navigation}
                </header>
                <main class="main-content"><div class="page-frame"><crate::domain_types::with_owner::alert::AdminAlert variant=crate::domain_types::with_owner::alert::AdminAlertVariant::Success id="saved">"Changes saved."</crate::domain_types::with_owner::alert::AdminAlert><div inner_html=String::from(content)></div></div></main>
            </div>
        },
    )
}
