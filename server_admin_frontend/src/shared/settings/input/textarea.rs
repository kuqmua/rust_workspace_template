#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the settings textarea is composed by the setting input dispatcher"
)]

use leptos::prelude::{ElementChild, OnAttribute};

pub(super) fn admin_setting_textarea(
    field: server_admin_contract::AdminSetting,
    value: super::LeptosAdminSettingSignal,
    disabled: super::AdminSettingDisabled,
    required: super::AdminSettingRequired,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let label = spec.label().as_ref().to_owned();
    let name = spec.name().as_ref().to_owned();
    let value = value.0;
    leptos::view! {
        <label><span>{label}</span><textarea
            name=name
            required=bool::from(required)
            disabled=bool::from(disabled)
            on:input=move |event| leptos::prelude::Set::set(
                &value,
                leptos::prelude::event_target_value(&event),
            )
        >{leptos::prelude::Get::get(&value)}</textarea></label>
    }
}
