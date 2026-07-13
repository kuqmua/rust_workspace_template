#![allow(unused_crate_dependencies)] // WASM dependencies are consumed by the library target mounted by this binary
#[cfg(target_arch = "wasm32")]
fn main() {
    leptos::mount::mount_to_body(server_admin_frontend::app::App);
}
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::missing_const_for_fn)] // Cargo checks the Trunk binary for the native host too
fn main() {}
