#[allow(clippy::single_call_fn)] // compatibility facade has one route-composition owner
pub(crate) fn admin_html_session_action_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(crate::AdminHtmlSessionActionRouteRegistry::router())
}
