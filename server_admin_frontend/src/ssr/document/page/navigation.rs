#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the administrator navigation is composed once by the document page shell"
)]

use leptos::prelude::{AriaAttributes, ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_nav(
    page: server_admin_contract::AdminPage,
    admin: Option<&server_admin_contract::AuthenticatedAdmin>,
    active_table: Option<server_admin_contract::AdminDataTable>,
) -> impl leptos::prelude::IntoView + use<> {
    let tables = server_admin_contract::AdminDataTable::PG_ORDER
        .into_iter()
        .filter(|table| {
            admin.is_none_or(|value| {
                bool::from(
                    value.has_permission(server_admin_contract::AdminPermission::TablesRead),
                ) && bool::from(value.has_permission(table.permission()))
            })
        })
        .map(|table| {
            let name = table.to_string();
            let href = table.frontend_path().to_string();
            leptos::view! {
                <crate::ui::navigation::AdminSidebarItem><crate::ui::navigation::AdminNavigationLink active=active_table == Some(table) href=href>{name}</crate::ui::navigation::AdminNavigationLink></crate::ui::navigation::AdminSidebarItem>
            }
        })
        .collect::<Vec<_>>();
    let pages = server_admin_contract::AdminPage::navigation()
        .filter(|item_page| admin.is_none_or(|value| bool::from(value.can_access(*item_page))))
        .map(|item_page| {
            let item = item_page.spec();
            let href = String::from(item.path());
            let label = item.route_name().as_ref().to_owned();
            leptos::view! {
                <crate::ui::navigation::AdminSidebarItem><crate::ui::navigation::AdminNavigationLink active=item_page == page href=href>{label}</crate::ui::navigation::AdminNavigationLink></crate::ui::navigation::AdminSidebarItem>
            }
        })
        .collect::<Vec<_>>();
    leptos::view! {
        <crate::ui::navigation::AdminSidebar>
            {tables}
            {pages}
            <crate::ui::navigation::AdminSidebarItem><form method="post" action=server_admin_contract::AdminHtmlAction::SignOut.get()><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary>{server_admin_contract::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</crate::ui::button::AdminButton></form></crate::ui::navigation::AdminSidebarItem>
        </crate::ui::navigation::AdminSidebar>
    }
}
