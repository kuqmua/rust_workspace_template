#![allow(
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos settings inputs convert signal values for event closures and are composed by the settings form"
)]

pub(crate) fn admin_setting_input(
    field: server_admin_contract::domain_types::AdminSetting,
    value: crate::domain_types::with_owner::input::LeptosAdminInputSignal,
    disabled: super::AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let required = super::AdminSettingRequired::from(bool::from(spec.required()));
    match spec.input_kind() {
        server_admin_contract::domain_types::AdminSettingInputKind::Text
        | server_admin_contract::domain_types::AdminSettingInputKind::Url => {
            leptos::prelude::IntoAny::into_any(super::admin_setting_text::admin_setting_text(
                field, value, disabled, required,
            ))
        }
        server_admin_contract::domain_types::AdminSettingInputKind::TextArea => {
            leptos::prelude::IntoAny::into_any(
                super::admin_setting_textarea::admin_setting_textarea(
                    field, value, disabled, required,
                ),
            )
        }
    }
}
