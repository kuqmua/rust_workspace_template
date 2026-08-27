#![allow(
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos settings inputs convert signal values for event closures and are composed by the settings form"
)]

#[path = "admin_setting_disabled.rs"]
mod admin_setting_disabled;
#[path = "admin_setting_input.rs"]
mod admin_setting_input;
#[path = "admin_setting_inputs.rs"]
mod admin_setting_inputs;
#[path = "admin_setting_required.rs"]
mod admin_setting_required;
#[path = "admin_setting_text.rs"]
mod admin_setting_text;
#[path = "admin_setting_textarea.rs"]
mod admin_setting_textarea;

pub(crate) use admin_setting_disabled::AdminSettingDisabled;
pub(crate) use admin_setting_input::admin_setting_input;
pub(crate) use admin_setting_inputs::admin_setting_inputs;
pub(super) use admin_setting_required::AdminSettingRequired;

impl crate::domain_types::with_owner::input::LeptosAdminInputSignal {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn value(self) -> super::values::admin_setting_input_value::AdminSettingInputValue {
        super::values::admin_setting_input_value::AdminSettingInputValue::from(
            leptos::prelude::Get::get(&self.signal()).into_boxed_str(),
        )
    }
}
