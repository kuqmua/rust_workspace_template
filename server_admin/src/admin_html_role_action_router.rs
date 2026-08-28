pub(crate) fn admin_html_role_action_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(crate::AdminHtmlRoleActionRouteRegistry::router())
}
