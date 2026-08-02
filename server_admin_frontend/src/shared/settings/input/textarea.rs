#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the settings textarea is composed by the setting input dispatcher"
)]

pub(super) fn admin_setting_textarea(
    field: server_admin_contract::AdminSetting,
    value: crate::ui::input::LeptosAdminInputSignal,
    disabled: super::AdminSettingDisabled,
    required: super::AdminSettingRequired,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let label = spec.label().as_ref().to_owned();
    leptos::view! {
        <crate::ui::field::AdminField label=label><crate::ui::textarea::AdminTextarea
            name=spec.name()
            required=bool::from(required)
            disabled=bool::from(disabled)
            bind_value=value
        /></crate::ui::field::AdminField>
    }
}
