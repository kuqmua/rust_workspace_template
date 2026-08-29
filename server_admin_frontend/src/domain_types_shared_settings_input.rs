pub(crate) use super::admin_setting_disabled::AdminSettingDisabled;
pub(crate) use super::admin_setting_inputs::admin_setting_inputs;
pub(super) use super::admin_setting_required::AdminSettingRequired;
impl crate::domain_types::with_owner::input::LeptosAdminInputSignal {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn value(self) -> super::values::admin_setting_input_value::AdminSettingInputValue {
        super::values::admin_setting_input_value::AdminSettingInputValue::from(
            leptos::prelude::Get::get(&self.signal()).into_boxed_str(),
        )
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod admin_setting_disabled {
    pub use super::super::admin_setting_disabled::*;
}
pub(crate) mod admin_setting_inputs {
    pub use super::super::admin_setting_inputs::*;
}
pub(crate) mod admin_setting_required {
    pub use super::super::admin_setting_required::*;
}
