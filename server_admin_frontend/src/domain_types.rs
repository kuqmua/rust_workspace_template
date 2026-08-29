#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_arch = "wasm32"))]
// Root-owned module compatibility wrappers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod admin_assets_error {}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod axum_admin_frontend_router {}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod routes {}
pub(crate) mod shared {}
#[cfg(not(target_arch = "wasm32"))]
pub mod ssr {}
#[cfg(target_arch = "wasm32")]
pub(crate) mod start {}
pub(crate) mod with_owner {}
