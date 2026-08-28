#![allow(clippy::module_inception)] // the routes function owns the required same-named module
#[path = "admin_api_open_api.rs"]
mod admin_api_open_api;
#[path = "admin_auth_route_registry.rs"]
mod admin_auth_route_registry;
#[must_use]
pub fn routes(state: super::SharedAdminAuthSvcStateArc) -> super::AxumAdminAuthRouter {
    let base_router = AdminAuthRouteRegistry::registry_router()
        .method_not_allowed_fallback(async || super::AdminError::MethodNotAllowed);
    let router = match <server_admin_contract::domain_types::AdminAuthenticationRouteFamily as frontend_contract::domain_types::RouteFamily>::body_limit() {
        Some(limit) => base_router.layer(axum::extract::DefaultBodyLimit::max(limit.get())),
        None => base_router,
    };
    super::AxumAdminAuthRouter(router.with_state(state))
}

pub use admin_api_open_api::admin_api_open_api;
use admin_auth_route_registry::AdminAuthRouteRegistry;
