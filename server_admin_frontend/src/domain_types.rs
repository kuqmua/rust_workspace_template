#[cfg(not(target_arch = "wasm32"))]
pub use super::axum_admin_frontend_router::AxumAdminFrontendRouter;
#[cfg(not(target_arch = "wasm32"))]
pub use super::routes::admin_frontend_routes;
// Root-owned module compatibility wrappers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod admin_assets_error {
    pub use super::super::admin_assets_error::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod axum_admin_frontend_router {
    pub use super::super::axum_admin_frontend_router::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod routes {
    pub use super::super::routes::*;
}
pub(crate) mod shared {
    pub use super::super::shared::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub mod ssr {
    pub use super::super::ssr::*;
}
#[cfg(target_arch = "wasm32")]
pub(crate) mod start {
    pub use super::super::start::*;
}
pub(crate) mod with_owner {
    pub use super::super::with_owner::*;
}
