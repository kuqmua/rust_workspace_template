pub(in crate::domain_types::auth::html) fn admin_html_action_router()
-> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(
        super::AdminHtmlActionRouteRegistry::registry_router()
            .merge(axum::Router::from(
                super::auth::admin_html_auth_action_router(),
            ))
            .merge(axum::Router::from(
                super::roles::admin_html_role_action_router(),
            ))
            .merge(axum::Router::from(
                super::sessions::admin_html_session_action_router(),
            ))
            .merge(axum::Router::from(
                super::settings::admin_html_settings_action_router(),
            ))
            .merge(axum::Router::from(
                super::users::admin_html_user_action_router(),
            )),
    )
}
