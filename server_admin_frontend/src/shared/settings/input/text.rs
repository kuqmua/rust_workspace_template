#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the settings text input is composed by the setting input dispatcher"
)]

use leptos::prelude::{ElementChild, OnAttribute};

pub(super) fn admin_setting_text(
    field: server_admin_contract::AdminSetting,
    value: super::LeptosAdminSettingSignal,
    disabled: super::AdminSettingDisabled,
    required: super::AdminSettingRequired,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let label = spec.label().as_ref().to_owned();
    let name = spec.name().as_ref().to_owned();
    let input_type = match spec.input_kind() {
        server_admin_contract::AdminSettingInputKind::Url => str_constants::HTML_URL_INPUT_TYPE,
        server_admin_contract::AdminSettingInputKind::Text
        | server_admin_contract::AdminSettingInputKind::TextArea => {
            str_constants::HTML_TEXT_INPUT_TYPE
        }
    };
    let value = value.0;
    leptos::view! {
        <label><span>{label}</span><input
            name=name
            type=input_type
            required=bool::from(required)
            disabled=bool::from(disabled)
            value=leptos::prelude::Get::get(&value)
            on:input=move |event| leptos::prelude::Set::set(
                &value,
                leptos::prelude::event_target_value(&event),
            )
        /></label>
    }
}
