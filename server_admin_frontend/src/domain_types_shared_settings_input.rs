impl crate::leptos_admin_input_signal::LeptosAdminInputSignal {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn value(self) -> super::values::admin_setting_input_value::AdminSettingInputValue {
        super::values::admin_setting_input_value::AdminSettingInputValue::from(
            leptos::prelude::Get::get(&self.signal()).into_boxed_str(),
        )
    }
}

// Root-owned module compatibility wrappers.
