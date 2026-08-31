pub(crate) fn html_routes(
    state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    swagger_enabled: crate::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled,
) -> crate::axum_admin_auth_router::AxumAdminAuthRouter {
    let router = crate::admin_html_page_route_registry::AdminHtmlPageRouteRegistry::router().merge(
        crate::admin_html_action_route_registry::AdminHtmlActionRouteRegistry::router()
            .merge(crate::admin_html_auth_action_route_registry::AdminHtmlAuthActionRouteRegistry::router())
            .merge(crate::admin_html_role_action_route_registry::AdminHtmlRoleActionRouteRegistry::router())
            .merge(crate::admin_html_session_action_route_registry::AdminHtmlSessionActionRouteRegistry::router())
            .merge(crate::admin_html_settings_action_route_registry::AdminHtmlSettingsActionRouteRegistry::router())
            .merge(crate::admin_html_user_action_route_registry::AdminHtmlUserActionRouteRegistry::router()),
    );
    let router = if *swagger_enabled.get_inner() {
        router.merge(
            crate::admin_html_swagger_route_registry::AdminHtmlSwaggerRouteRegistry::router(),
        )
    } else {
        router
    };
    crate::axum_admin_auth_router::AxumAdminAuthRouter::from(router.with_state(state))
}
