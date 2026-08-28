pub(crate) fn admin_html_user_action_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(
        crate::admin_html_user_action_route_registry::AdminHtmlUserActionRouteRegistry::router(),
    )
}
