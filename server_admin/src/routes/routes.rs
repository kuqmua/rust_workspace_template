#[must_use]
pub fn routes(
    state: super::super::SharedAdminAuthSvcStateArc,
) -> super::super::AxumAdminAuthRouter {
    let base_router = super::AdminAuthRouteRegistry::registry_router()
        .method_not_allowed_fallback(async || super::super::AdminError::MethodNotAllowed);
    let router = match <server_admin_contract::domain_types::AdminAuthenticationRouteFamily as frontend_contract::domain_types::RouteFamily>::body_limit() {
        Some(limit) => base_router.layer(axum::extract::DefaultBodyLimit::max(limit.get())),
        None => base_router,
    };
    super::super::AxumAdminAuthRouter(router.with_state(state))
}
