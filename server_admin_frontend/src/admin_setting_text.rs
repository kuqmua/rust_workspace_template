#![allow(
    clippy::unused_trait_names,
    reason = "the settings text input is composed by the setting input dispatcher"
)]

pub(super) fn admin_setting_text(
    field: server_admin_contract::domain_types::AdminSetting,
    value: crate::domain_types::with_owner::input::LeptosAdminInputSignal,
    disabled: super::AdminSettingDisabled,
    required: super::AdminSettingRequired,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let label = spec.label().as_ref().to_owned();
    let input_kind = match spec.input_kind() {
        server_admin_contract::domain_types::AdminSettingInputKind::Url => {
            crate::domain_types::with_owner::input::AdminInputKind::Url
        }
        server_admin_contract::domain_types::AdminSettingInputKind::Text
        | server_admin_contract::domain_types::AdminSettingInputKind::TextArea => {
            crate::domain_types::with_owner::input::AdminInputKind::Text
        }
    };
    leptos::view! {
        <crate::domain_types::with_owner::field::AdminField label=label><crate::domain_types::with_owner::input::AdminInput
            name=spec.name()
            kind=input_kind
            required=bool::from(required)
            disabled=bool::from(disabled)
            bind_value=value
        /></crate::domain_types::with_owner::field::AdminField>
    }
}
