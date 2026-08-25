#![allow(
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos component and entry-point macro expansion produces these patterns"
)]

mod admin_app;
mod admin_csr_query;
mod admin_data_grid;
mod admin_nav;
mod admin_pagination;
mod admin_permissions_view;
mod admin_profile_view;
mod admin_roles_view;
mod admin_sessions_view;
mod admin_settings_view;
mod admin_users_view;
mod fetch_page;
mod http;
mod mutation;
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
