pub(crate) use update_settings::AdminHtmlSettingsActionRouteRegistry;

// Root-owned module compatibility wrappers.
mod update_settings {
    pub use super::super::update_settings::*;
}
