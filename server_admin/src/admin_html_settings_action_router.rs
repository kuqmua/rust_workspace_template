pub(in crate::domain_types::auth::html::actions) fn admin_html_settings_action_router()
-> super::super::super::super::AxumAdminStateRouter {
    super::super::super::super::AxumAdminStateRouter::from(
        super::AdminHtmlSettingsActionRouteRegistry::registry_router(),
    )
}
