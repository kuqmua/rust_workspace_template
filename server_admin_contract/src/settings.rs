#[path = "admin_branding_view.rs"]
mod admin_branding_view;
#[path = "admin_optional_setting.rs"]
mod admin_optional_setting;
#[path = "admin_setting.rs"]
mod admin_setting;
#[path = "admin_setting_input_kind.rs"]
mod admin_setting_input_kind;
#[path = "admin_setting_label.rs"]
mod admin_setting_label;
#[path = "admin_setting_name.rs"]
mod admin_setting_name;
#[path = "admin_setting_optionality.rs"]
mod admin_setting_optionality;
#[path = "admin_setting_spec.rs"]
mod admin_setting_spec;
#[path = "admin_settings_view.rs"]
mod admin_settings_view;
#[path = "admin_update_settings_req.rs"]
mod admin_update_settings_req;

pub use admin_branding_view::AdminBrandingView;
pub use admin_optional_setting::*;
pub use admin_setting::*;
pub use admin_setting_input_kind::AdminSettingInputKind;
pub use admin_setting_label::AdminSettingLabel;
pub use admin_setting_name::AdminSettingName;
pub use admin_setting_optionality::AdminSettingOptionality;
pub use admin_setting_spec::AdminSettingSpec;
pub use admin_settings_view::*;
pub use admin_update_settings_req::AdminUpdateSettingsReq;

#[cfg(test)]
#[path = "domain_types_settings_tests.rs"]
mod tests;
