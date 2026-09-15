#[must_use]
#[allow(
    clippy::module_name_repetitions,
    reason = "the established public API names the administrator route collection explicitly"
)]
pub fn admin_auth_routes(
    shared_admin_auth_svc_state_arc: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
) -> crate::axum_admin_auth_router::AxumAdminAuthRouter {
    crate::axum_admin_auth_router::AxumAdminAuthRouter::from(
        crate::admin_auth_route_registry::router_with_body_limit()
            .with_state(shared_admin_auth_svc_state_arc),
    )
}
