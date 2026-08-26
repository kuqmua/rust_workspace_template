#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the settings value collection keeps its conversion adjacent to indexed access"
)]

#[path = "values_admin_setting_input_value.rs"]
pub(crate) mod admin_setting_input_value;
#[path = "values_admin_settings_form_values.rs"]
pub(crate) mod admin_settings_form_values;
