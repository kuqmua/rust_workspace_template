#![allow(
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos component and entry-point macro expansion produces these patterns"
)]

#[path = "admin_app.rs"]
mod admin_app;
#[path = "admin_csr_query.rs"]
mod admin_csr_query;
#[path = "admin_data_grid.rs"]
mod admin_data_grid;
#[path = "domain_types_start_admin_nav.rs"]
mod admin_nav;
#[path = "admin_pagination.rs"]
mod admin_pagination;
#[path = "admin_permissions_view.rs"]
mod admin_permissions_view;
#[path = "admin_profile_view.rs"]
mod admin_profile_view;
#[path = "admin_roles_view.rs"]
mod admin_roles_view;
#[path = "admin_sessions_view.rs"]
mod admin_sessions_view;
#[path = "admin_settings_view.rs"]
mod admin_settings_view;
#[path = "admin_users_view.rs"]
mod admin_users_view;
#[path = "fetch_page.rs"]
mod fetch_page;
#[path = "http.rs"]
mod http;
#[path = "domain_types_start_mutation.rs"]
mod mutation;
#[path = "state.rs"]
mod state;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub(crate) fn start() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(element) = document.get_element_by_id(constants_str::ADMIN_CSR_ROOT_ID) else {
        return;
    };
    let Ok(root) = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(element) else {
        return;
    };
    root.set_inner_html(constants_str::EMPTY);
    leptos::mount::mount_to(root, admin_app::AdminApp).forget();
}
