#[cfg(not(target_arch = "wasm32"))]
#[path = "admin_assets_error.rs"]
mod admin_assets_error;
#[cfg(not(target_arch = "wasm32"))]
#[path = "axum_admin_frontend_router.rs"]
mod axum_admin_frontend_router;
#[cfg(not(target_arch = "wasm32"))]
#[path = "routes.rs"]
mod routes;
#[path = "shared.rs"]
mod shared;
#[cfg(not(target_arch = "wasm32"))]
#[path = "ssr.rs"]
pub mod ssr;
#[cfg(target_arch = "wasm32")]
#[path = "start.rs"]
mod start;
#[path = "with_owner.rs"]
mod with_owner;

#[cfg(not(target_arch = "wasm32"))]
pub use axum_admin_frontend_router::AxumAdminFrontendRouter;
#[cfg(not(target_arch = "wasm32"))]
pub use routes::routes;
