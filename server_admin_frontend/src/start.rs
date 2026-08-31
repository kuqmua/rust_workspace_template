#![allow(
    clippy::same_name_method,
    clippy::shadow_reuse,
    clippy::unused_trait_names,
    reason = "Leptos component and entry-point macro expansion produces these patterns"
)]

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

// Root-owned module compatibility wrappers.
pub(crate) mod admin_app {}
pub(crate) mod admin_csr_query {}
pub(crate) mod admin_data_grid {}
pub(crate) mod admin_pagination {}
pub(crate) mod admin_permissions_view {}
pub(crate) mod admin_profile_view {}
pub(crate) mod admin_roles_view {}
pub(crate) mod admin_sessions_view {}
pub(crate) mod admin_settings_view {}
pub(crate) mod admin_users_view {}
pub(crate) mod csr_admin_nav {}
pub(crate) mod fetch_page {}
pub(crate) mod http {}
pub(crate) mod mutation {}
pub(crate) mod state {}
