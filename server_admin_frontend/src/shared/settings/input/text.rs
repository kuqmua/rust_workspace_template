#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the settings text input is composed by the setting input dispatcher"
)]

pub(super) fn admin_setting_text(
    field: server_admin_contract::AdminSetting,
    value: crate::ui::input::LeptosAdminInputSignal,
    disabled: super::AdminSettingDisabled,
    required: super::AdminSettingRequired,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let label = spec.label().as_ref().to_owned();
    let input_kind = match spec.input_kind() {
        server_admin_contract::AdminSettingInputKind::Url => crate::ui::input::AdminInputKind::Url,
        server_admin_contract::AdminSettingInputKind::Text
        | server_admin_contract::AdminSettingInputKind::TextArea => {
            crate::ui::input::AdminInputKind::Text
        }
    };
    leptos::view! {
        <crate::ui::field::AdminField label=label><crate::ui::input::AdminInput
            name=spec.name()
            kind=input_kind
            required=bool::from(required)
            disabled=bool::from(disabled)
            bind_value=value
        /></crate::ui::field::AdminField>
    }
}
