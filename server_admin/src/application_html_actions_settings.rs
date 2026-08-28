#[path = "admin_html_settings_action_router.rs"]
mod admin_html_settings_action_router;
#[path = "update_settings.rs"]
mod update_settings;

pub(super) use admin_html_settings_action_router::admin_html_settings_action_router;
pub(super) use update_settings::AdminHtmlSettingsActionRouteRegistry;
