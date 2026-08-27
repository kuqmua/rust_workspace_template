use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(in crate::domain_types::start) fn CsrAdminNav(
    admin: Option<server_admin_contract::domain_types::AuthenticatedAdmin>,
) -> impl leptos::prelude::IntoView {
    let pathname = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_default();
    let active_table = super::admin_csr_query::AdminCsrQuery::from_location()
        .ok()
        .and_then(|query| query.table);
    leptos::view! {
        <header class="topbar"><crate::domain_types::with_owner::navigation::admin_sidebar::AdminSidebar>
            {admin.as_ref().map_or_else(Vec::new, |admin| server_admin_contract::domain_types::AdminDataTable::PG_ORDER.into_iter().filter(|table| {
                    bool::from(admin.has_permission(server_admin_contract::domain_types::AdminPermission::TablesRead))
                        && bool::from(admin.has_permission(table.permission()))
                }).map(|table| {
                    let name = table.to_string();
                    let href = table.frontend_path().to_string();
                    leptos::view! { <crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem><crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink active=active_table == Some(table) href=href>{name}</crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink></crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem> }
                }).collect::<Vec<_>>())}
            {admin.as_ref().map_or_else(Vec::new, |admin| server_admin_contract::domain_types::AdminPage::navigation().filter(|page| {
                    bool::from(admin.can_access(*page))
                }).map(|page| {
                    let spec = page.spec();
                    let href = spec.path().as_ref().to_owned();
                    let active = pathname == href;
                    let label = spec.route_name().as_ref().to_owned();
                    leptos::view! { <crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem><crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink active=active href=href>{label}</crate::domain_types::with_owner::navigation::admin_navigation_link::AdminNavigationLink></crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem> }
                }).collect::<Vec<_>>())}
            <crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem><form on:submit=move |event| {
                event.prevent_default();
                if let Ok(path) = super::http::url::admin_api_url(server_admin_contract::domain_types::AdminRoute::SignOut) {
                    super::mutation::reload_after(
                        super::mutation::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::domain_types::AdminNoBody,
                    );
                }
            }><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>{server_admin_contract::domain_types::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</crate::domain_types::with_owner::button::AdminButton></form></crate::domain_types::with_owner::navigation::admin_sidebar_item::AdminSidebarItem>
        </crate::domain_types::with_owner::navigation::admin_sidebar::AdminSidebar></header>
    }
}
