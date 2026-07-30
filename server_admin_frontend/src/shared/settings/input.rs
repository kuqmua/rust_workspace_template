#![allow(
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos settings inputs convert signal values for event closures and are composed by the settings form"
)]

use leptos::prelude::{ElementChild, OnAttribute};

#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub(crate) struct AdminSettingDisabled(bool);
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct AdminSettingRequired(bool);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct LeptosAdminSettingSignal(leptos::prelude::RwSignal<String>);
impl LeptosAdminSettingSignal {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn value(self) -> super::values::AdminSettingInputValue {
        super::values::AdminSettingInputValue::from(
            leptos::prelude::Get::get(&self.0).into_boxed_str(),
        )
    }
}

pub(crate) fn admin_setting_input(
    field: server_admin_contract::AdminSetting,
    value: LeptosAdminSettingSignal,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let label = spec.label().as_ref().to_owned();
    let name = spec.name().as_ref().to_owned();
    let required = bool::from(spec.required());
    let disabled = bool::from(disabled);
    let value = value.0;
    match spec.input_kind() {
        server_admin_contract::AdminSettingInputKind::Text
        | server_admin_contract::AdminSettingInputKind::Url => {
            let input_type = match spec.input_kind() {
                server_admin_contract::AdminSettingInputKind::Url => {
                    str_constants::HTML_URL_INPUT_TYPE
                }
                server_admin_contract::AdminSettingInputKind::Text
                | server_admin_contract::AdminSettingInputKind::TextArea => {
                    str_constants::HTML_TEXT_INPUT_TYPE
                }
            };
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <label><span>{label}</span><input
                    name=name
                    type=input_type
                    required=required
                    disabled=disabled
                    value=leptos::prelude::Get::get(&value)
                    on:input=move |event| leptos::prelude::Set::set(
                        &value,
                        leptos::prelude::event_target_value(&event),
                    )
                /></label>
            })
        }
        server_admin_contract::AdminSettingInputKind::TextArea => {
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <label><span>{label}</span><textarea
                    name=name
                    required=required
                    disabled=disabled
                    on:input=move |event| leptos::prelude::Set::set(
                        &value,
                        leptos::prelude::event_target_value(&event),
                    )
                >{leptos::prelude::Get::get(&value)}</textarea></label>
            })
        }
    }
}

pub(crate) fn admin_setting_inputs(
    signals: super::signals::AdminSettingsFormSignals,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {server_admin_contract::AdminSetting::ALL.into_iter().map(|setting| {
            admin_setting_input(setting, signals.get(setting), disabled)
        }).collect::<Vec<_>>()}
    }
}
