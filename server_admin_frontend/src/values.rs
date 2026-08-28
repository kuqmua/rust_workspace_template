#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the settings value collection keeps its conversion adjacent to indexed access"
)]

// Root-owned module compatibility wrappers.
pub(crate) mod admin_setting_input_value {
    pub use crate::admin_setting_input_value::*;
}
pub(crate) mod admin_settings_form_values {
    pub use crate::admin_settings_form_values::*;
}
