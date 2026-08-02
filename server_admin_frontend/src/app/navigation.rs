use leptos::prelude::{AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(in crate::app) fn AdminNav(
    admin: server_admin_contract::AuthenticatedAdmin,
) -> impl leptos::prelude::IntoView {
    let pathname = web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .unwrap_or_default();
    let active_table = super::query::AdminCsrQuery::from_location()
        .ok()
        .and_then(|query| query.table);
    leptos::view! {
        <header class="topbar"><nav data-name="NavigationMenu" class="relative z-10 flex max-w-max flex-1 items-center justify-center" aria-label="Admin sections">
            {server_admin_contract::AdminDataTable::PG_ORDER.into_iter().filter(|table| {
                bool::from(admin.has_permission(server_admin_contract::AdminPermission::TablesRead))
                    && bool::from(admin.has_permission(table.permission()))
            }).map(|table| {
                let name = table.to_string();
                let href = table.frontend_path().to_string();
                leptos::view! { <crate::ui::navigation::AdminNavigationLink active=active_table == Some(table) href=href>{name}</crate::ui::navigation::AdminNavigationLink> }
            }).collect::<Vec<_>>()}
            {server_admin_contract::AdminPage::navigation().filter(|page| {
                bool::from(admin.can_access(*page))
            }).map(|page| {
                let spec = page.spec();
                let href = spec.path().as_ref().to_owned();
                let active = pathname == href;
                let label = spec.route_name().as_ref().to_owned();
                leptos::view! { <crate::ui::navigation::AdminNavigationLink active=active href=href>{label}</crate::ui::navigation::AdminNavigationLink> }
            }).collect::<Vec<_>>()}
            <form on:submit=move |event| {
                event.prevent_default();
                if let Ok(path) = super::http::url::admin_api_url(server_admin_contract::AdminRoute::SignOut) {
                    super::mutation::reload_after(
                        super::mutation::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::AdminNoBody,
                    );
                }
            }><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary>{server_admin_contract::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</crate::ui::button::AdminButton></form>
        </nav></header>
    }
}
