#[path = "application_html_actions_settings/router.rs"]
mod router;
#[path = "application_html_actions_settings/update_settings.rs"]
mod update_settings;

pub(super) use router::router;
pub(super) use update_settings::AdminHtmlSettingsActionRouteRegistry;
