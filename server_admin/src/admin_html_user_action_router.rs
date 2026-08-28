pub(in super::super) fn admin_html_user_action_router()
-> super::super::super::super::AxumAdminStateRouter {
    super::super::super::super::AxumAdminStateRouter::from(
        super::admin_html_user_action_route_registry::AdminHtmlUserActionRouteRegistry::router(),
    )
}
