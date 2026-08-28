pub(in super::super::super) fn admin_html_auth_action_router()
-> super::super::super::super::super::AxumAdminStateRouter {
    super::super::super::super::super::AxumAdminStateRouter::from(
        super::AdminHtmlAuthActionRouteRegistry::router(),
    )
}
