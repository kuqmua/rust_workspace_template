pub(crate) fn admin_html_auth_action_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(crate::AdminHtmlAuthActionRouteRegistry::router())
}
