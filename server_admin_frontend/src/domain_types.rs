#[cfg(not(target_arch = "wasm32"))]
pub use crate::axum_admin_frontend_router::AxumAdminFrontendRouter;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::routes::admin_frontend_routes;

// Root-owned module compatibility wrappers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod admin_assets_error {
    pub use crate::admin_assets_error::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod axum_admin_frontend_router {
    pub use crate::axum_admin_frontend_router::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod routes {
    pub use crate::routes::*;
}
pub(crate) mod shared {
    pub use crate::shared::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub mod ssr {
    pub use crate::ssr::*;
}
#[cfg(target_arch = "wasm32")]
pub(crate) mod start {
    pub use crate::start::*;
}
pub(crate) mod with_owner {
    pub use crate::with_owner::*;
}
