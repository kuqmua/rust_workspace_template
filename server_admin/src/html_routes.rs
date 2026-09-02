pub(crate) fn html_routes(
    shared_admin_auth_svc_state_arc: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    admin_html_swagger_enabled: crate::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled,
) -> crate::axum_admin_auth_router::AxumAdminAuthRouter {
    let router = crate::admin_html_page_route_registry::router().merge(
        crate::admin_html_action_route_registry::router()
            .merge(crate::admin_html_auth_action_route_registry::router())
            .merge(crate::admin_html_role_action_route_registry::router())
            .merge(crate::admin_html_session_action_route_registry::router())
            .merge(crate::admin_html_settings_action_route_registry::router())
            .merge(crate::admin_html_user_action_route_registry::router()),
    );
    let router = if *admin_html_swagger_enabled.get_inner() {
        router.merge(crate::admin_html_swagger_route_registry::router())
    } else {
        router
    };
    crate::axum_admin_auth_router::AxumAdminAuthRouter::from(
        router.with_state(shared_admin_auth_svc_state_arc),
    )
}
