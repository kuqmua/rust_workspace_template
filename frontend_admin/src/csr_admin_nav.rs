#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(crate) fn CsrAdminNav(
    admin_bool: server_admin_contract::admin_bool::AdminBool,
    option: Option<server_admin_contract::authenticated_admin::AuthenticatedAdmin>,
) -> impl leptos::prelude::IntoView {
    let pathname = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_default();
    let admin_page_path_ref =
        server_admin_contract::admin_page_path_ref::AdminPagePathRef::from(pathname.as_str());
    let tables = option.as_ref().map_or_else(Vec::new, |option| server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER.into_iter().filter(|table| {
                    !bool::from(admin_bool) && bool::from(option.has_rule(server_admin_contract::admin_rule::AdminRule::TablesRead))
                        && bool::from(option.has_rule(table.rule()))
                }).map(|table| {
                    let name = table.to_string();
                    let href = table.frontend_path().to_string();
                    let active = bool::from(admin_page_path_ref.is_navigation_section(&table.frontend_path()));
                    leptos::view! { <crate::admin_sidebar_item::AdminSidebarItem><crate::admin_navigation_link::AdminNavigationLink bool=active string=href>{name}</crate::admin_navigation_link::AdminNavigationLink></crate::admin_sidebar_item::AdminSidebarItem> }
                }).collect::<Vec<_>>());
    let pages = option.as_ref().map_or_else(Vec::new, |option| server_admin_contract::admin_page::AdminPage::navigation().filter(|page| {
                    (!bool::from(admin_bool) || *page == server_admin_contract::admin_page::AdminPage::Profile) && bool::from(option.can_access(*page))
                }).map(|page| {
                    let spec = page.spec();
                    let href = spec.path().as_ref().to_owned();
                    let active = bool::from(admin_page_path_ref.is_navigation_section(&server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath::from(spec.frontend_path())));
                    let label = spec.route_name().as_ref().to_owned();
                    leptos::view! { <crate::admin_sidebar_item::AdminSidebarItem><crate::admin_navigation_link::AdminNavigationLink bool=active string=href>{label}</crate::admin_navigation_link::AdminNavigationLink></crate::admin_sidebar_item::AdminSidebarItem> }
                }).collect::<Vec<_>>());
    leptos::view! {
        <header class="topbar"><crate::admin_sidebar::AdminSidebar>
            {tables}
            {pages}
            <crate::admin_sidebar_item::AdminSidebarItem><form on:submit=move |event| {
                event.prevent_default();
                if let Ok(path) = crate::admin_api_url::admin_api_url(server_admin_contract::admin_route::AdminRoute::SignOut) {
                    crate::reload_after::reload_after(
                        crate::admin_mutation_method::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::admin_no_body::AdminNoBody,
                    );
                }
            }><button type="submit">{server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</button></form></crate::admin_sidebar_item::AdminSidebarItem>
        </crate::admin_sidebar::AdminSidebar></header>
    }
}
