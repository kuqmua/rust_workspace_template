use leptos::prelude::{AriaAttributes, ClassAttribute, ElementChild, OnAttribute};

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
        <header class="topbar"><nav aria-label="Admin sections">
            {server_admin_contract::AdminDataTable::PG_ORDER.into_iter().filter(|table| {
                bool::from(admin.has_permission(server_admin_contract::AdminPermission::TablesRead))
                    && bool::from(admin.has_permission(table.permission()))
            }).map(|table| {
                let name = table.to_string();
                let href = table.frontend_path().to_string();
                leptos::view! { <a class=("active", active_table == Some(table)) href=href>{name}</a> }
            }).collect::<Vec<_>>()}
            {server_admin_contract::AdminPage::navigation().filter(|page| {
                bool::from(admin.can_access(*page))
            }).map(|page| {
                let spec = page.spec();
                let href = spec.path().as_ref().to_owned();
                let active = pathname == href;
                let label = spec.route_name().as_ref().to_owned();
                leptos::view! { <a class=("active", active) href=href>{label}</a> }
            }).collect::<Vec<_>>()}
            <form on:submit=move |event| {
                event.prevent_default();
                if let Ok(path) = super::http::admin_api_url(server_admin_contract::AdminRoute::SignOut) {
                    super::mutation::reload_after(
                        super::mutation::AdminMutationMethod::Post,
                        path,
                        server_admin_contract::AdminNoBody,
                    );
                }
            }><button type="submit">{server_admin_contract::AdminHtmlAction::SignOut.route_name().as_ref().to_owned()}</button></form>
        </nav></header>
    }
}
