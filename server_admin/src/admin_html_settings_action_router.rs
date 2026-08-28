pub(crate) fn admin_html_settings_action_router() -> crate::AxumAdminStateRouter {
    crate::AxumAdminStateRouter::from(
        crate::AdminHtmlSettingsActionRouteRegistry::registry_router(),
    )
}
