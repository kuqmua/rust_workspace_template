pub(crate) use admin_html_settings_action_router::admin_html_settings_action_router;
pub(crate) use update_settings::AdminHtmlSettingsActionRouteRegistry;

// Root-owned module compatibility wrappers.
mod admin_html_settings_action_router {
    pub use crate::admin_html_settings_action_router::*;
}
mod update_settings {
    pub use crate::update_settings::*;
}
