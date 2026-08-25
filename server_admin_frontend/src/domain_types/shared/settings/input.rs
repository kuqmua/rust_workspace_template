#![allow(
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos settings inputs convert signal values for event closures and are composed by the settings form"
)]

mod admin_setting_text;
mod admin_setting_textarea;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct AdminSettingDisabled(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
struct AdminSettingRequired(bool);
impl crate::domain_types::with_owner::input::LeptosAdminInputSignal {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn value(self) -> super::values::AdminSettingInputValue {
        super::values::AdminSettingInputValue::from(
            leptos::prelude::Get::get(&self.signal()).into_boxed_str(),
        )
    }
}

pub(crate) fn admin_setting_input(
    field: server_admin_contract::domain_types::AdminSetting,
    value: crate::domain_types::with_owner::input::LeptosAdminInputSignal,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let required = AdminSettingRequired::from(bool::from(spec.required()));
    match spec.input_kind() {
        server_admin_contract::domain_types::AdminSettingInputKind::Text
        | server_admin_contract::domain_types::AdminSettingInputKind::Url => {
            leptos::prelude::IntoAny::into_any(admin_setting_text::admin_setting_text(
                field, value, disabled, required,
            ))
        }
        server_admin_contract::domain_types::AdminSettingInputKind::TextArea => {
            leptos::prelude::IntoAny::into_any(admin_setting_textarea::admin_setting_textarea(
                field, value, disabled, required,
            ))
        }
    }
}

pub(crate) fn admin_setting_inputs(
    signals: super::admin_settings_form_signals::AdminSettingsFormSignals,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {server_admin_contract::domain_types::AdminSetting::ALL.into_iter().map(|setting| {
            admin_setting_input(setting, signals.get(setting), disabled)
        }).collect::<Vec<_>>()}
    }
}
