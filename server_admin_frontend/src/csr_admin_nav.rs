use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(crate) fn CsrAdminNav(
    admin: Option<server_admin_contract::authenticated_admin::AuthenticatedAdmin>,
) -> impl leptos::prelude::IntoView {
    let pathname = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_default();
    let active_table = super::admin_csr_query::AdminCsrQuery::from_location()
        .ok()
        .and_then(|query| query.table);
    leptos::view! {
        <header class="topbar"><crate::admin_sidebar::AdminSidebar>
            {admin.as_ref().map_or_else(Vec::new, |admin| server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER.into_iter().filter(|table| {
                    bool::from(admin.has_permission(server_admin_contract::admin_permission::AdminPermission::TablesRead))
                        && bool::from(admin.has_permission(table.permission()))
                }).map(|table| {
                    let name = table.to_string();
                    let href = table.frontend_path().to_string();
                    leptos::view! { <crate::admin_sidebar_item::AdminSidebarItem><crate::admin_navigation_link::AdminNavigationLink active=active_table == Some(table) href=href>{name}</crate::admin_navigation_link::AdminNavigationLink></crate::admin_sidebar_item::AdminSidebarItem> }
                }).collect::<Vec<_>>())}
            {admin.as_ref().map_or_else(Vec::new, |admin| server_admin_contract::admin_page::AdminPage::navigation().filter(|page| {
                    bool::from(admin.can_access(*page))
                }).map(|page| {
                    let spec = page.spec();
                    let href = spec.path().as_ref().to_owned();
                    let active = pathname == href;
                    let label = spec.route_name().as_ref().to_owned();
                    leptos::view! { <crate::admin_sidebar_item::AdminSidebarItem><crate::admin_navigation_link::AdminNavigationLink active=active href=href>{label}</crate::admin_navigation_link::AdminNavigationLink></crate::admin_sidebar_item::AdminSidebarItem> }
                }).collect::<Vec<_>>())}
            <crate::admin_sidebar_item::AdminSidebarItem><form on:submit=move |event| {
                event.prevent_default();
                if let Ok(path) = super::http::url::admin_api_url(server_admin_contract::admin_route::AdminRoute::SignOut) {
                    super::mutation::reload_after(
                        super::mutation::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::admin_no_body::AdminNoBody,
                    );
                }
            }><crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Secondary>{server_admin_contract::admin_html_action::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</crate::admin_button::AdminButton></form></crate::admin_sidebar_item::AdminSidebarItem>
        </crate::admin_sidebar::AdminSidebar></header>
    }
}
