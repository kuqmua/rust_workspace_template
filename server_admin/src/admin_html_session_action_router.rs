pub(crate) fn admin_html_session_action_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(crate::AdminHtmlSessionActionRouteRegistry::registry_router())
}
