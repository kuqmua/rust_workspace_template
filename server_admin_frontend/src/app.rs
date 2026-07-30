#![allow(
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos component and entry-point macro expansion produces these patterns"
)]

mod data_grid;
mod http;
mod loader;
mod mutation;
mod navigation;
mod pagination;
mod permissions;
mod profile;
mod query;
mod roles;
mod sessions;
mod settings;
mod shell;
mod state;
mod users;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub(crate) fn start() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(element) = document.get_element_by_id(str_constants::ADMIN_CSR_ROOT_ID) else {
        return;
    };
    let Ok(root) = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(element) else {
        return;
    };
    root.set_inner_html(str_constants::EMPTY);
    leptos::mount::mount_to(root, shell::AdminApp).forget();
}
