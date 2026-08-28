#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn admin_html_action_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(
        crate::AdminHtmlActionRouteRegistry::router()
            .merge(axum::Router::from(crate::admin_html_auth_action_router()))
            .merge(axum::Router::from(crate::admin_html_role_action_router()))
            .merge(axum::Router::from(crate::admin_html_session_action_router()))
            .merge(axum::Router::from(
                crate::admin_html_settings_action_router(),
            ))
            .merge(axum::Router::from(crate::admin_html_user_action_router())),
    )
}
