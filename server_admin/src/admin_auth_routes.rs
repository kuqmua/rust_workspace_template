#[must_use]
#[allow(
    clippy::module_name_repetitions,
    reason = "the established public API names the administrator route collection explicitly"
)]
pub fn admin_auth_routes(
    state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
) -> crate::axum_admin_auth_router::AxumAdminAuthRouter {
    let base_router = crate::admin_auth_route_registry::AdminAuthRouteRegistry::router()
        .method_not_allowed_fallback(async || crate::admin_error::AdminError::MethodNotAllowed);
    let router = match <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit() {
        Some(limit) => base_router.layer(axum::extract::DefaultBodyLimit::max(limit.get())),
        None => base_router,
    };
    crate::axum_admin_auth_router::AxumAdminAuthRouter::from(router.with_state(state))
}

// Root-owned module compatibility wrappers.
mod admin_api_open_api {}
mod admin_auth_route_registry {}
