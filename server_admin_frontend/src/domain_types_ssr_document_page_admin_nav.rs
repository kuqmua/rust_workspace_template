#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the administrator navigation is composed once by the document page shell"
)]

use leptos::prelude::{AriaAttributes, ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_nav(
    page: server_admin_contract::domain_types::AdminPage,
    admin: Option<&server_admin_contract::domain_types::AuthenticatedAdmin>,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> impl leptos::prelude::IntoView + use<> {
    let tables = server_admin_contract::domain_types::AdminDataTable::PG_ORDER
        .into_iter()
        .filter(|table| {
            admin.is_none_or(|value| {
                bool::from(
                    value.has_permission(server_admin_contract::domain_types::AdminPermission::TablesRead),
                ) && bool::from(value.has_permission(table.permission()))
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
        .filter(|item_page| admin.is_none_or(|value| bool::from(value.can_access(*item_page))))
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
}
