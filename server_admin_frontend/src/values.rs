#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the settings value collection keeps its conversion adjacent to indexed access"
)]

#[path = "admin_setting_input_value.rs"]
pub(crate) mod admin_setting_input_value;
#[path = "admin_settings_form_values.rs"]
pub(crate) mod admin_settings_form_values;
