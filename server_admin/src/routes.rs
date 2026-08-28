#![allow(clippy::module_inception)] // the routes function owns the required same-named module
#[must_use]
pub fn admin_auth_routes(state: crate::SharedAdminAuthSvcStateArc) -> crate::AxumAdminAuthRouter {
    let base_router = AdminAuthRouteRegistry::router()
        .method_not_allowed_fallback(async || crate::AdminError::MethodNotAllowed);
    let router = match <server_admin_contract::domain_types::AdminAuthenticationRouteFamily as frontend_contract::domain_types::RouteFamily>::body_limit() {
        Some(limit) => base_router.layer(axum::extract::DefaultBodyLimit::max(limit.get())),
        None => base_router,
    };
    crate::AxumAdminAuthRouter(router.with_state(state))
}

pub use admin_api_open_api::admin_api_open_api;
use admin_auth_route_registry::AdminAuthRouteRegistry;

// Root-owned module compatibility wrappers.
mod admin_api_open_api {
    pub use crate::admin_api_open_api::*;
}
mod admin_auth_route_registry {
    pub use crate::admin_auth_route_registry::*;
}
