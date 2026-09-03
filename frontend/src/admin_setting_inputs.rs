#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn admin_setting_inputs(
    admin_settings_form_signals: crate::admin_settings_form_signals::AdminSettingsFormSignals,
    admin_setting_disabled: crate::admin_setting_disabled::AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {server_admin_contract::admin_setting::AdminSetting::ALL.into_iter().map(|setting| {
                let value = admin_settings_form_signals.get(setting);
                    let spec = setting.spec();
                    let required = crate::admin_setting_required::AdminSettingRequired::from(bool::from(spec.required()));
                    match spec.input_kind() {
                        server_admin_contract::admin_setting_input_kind::AdminSettingInputKind::Text
                        | server_admin_contract::admin_setting_input_kind::AdminSettingInputKind::Url => {
                            let label = spec.label().as_ref().to_owned();
                            let input_kind = match spec.input_kind() {
                                server_admin_contract::admin_setting_input_kind::AdminSettingInputKind::Url => {
                                    crate::admin_input_kind::AdminInputKind::Url
                                }
                                server_admin_contract::admin_setting_input_kind::AdminSettingInputKind::Text
                                | server_admin_contract::admin_setting_input_kind::AdminSettingInputKind::TextArea => {
                                    crate::admin_input_kind::AdminInputKind::Text
                                }
                            };
                            leptos::prelude::IntoAny::into_any(leptos::view! {
                                <crate::admin_field::AdminField admin_field_label=label><crate::admin_input::AdminInput
                                    admin_input_name=spec.name()
                                    admin_input_kind=input_kind
                                    required=bool::from(required)
                                    disabled=bool::from(admin_setting_disabled)
                                    bind_value=value
                                /></crate::admin_field::AdminField>
                            })
                        }
                        server_admin_contract::admin_setting_input_kind::AdminSettingInputKind::TextArea => {
                            let label = spec.label().as_ref().to_owned();
                            leptos::prelude::IntoAny::into_any(leptos::view! {
                                <crate::admin_field::AdminField admin_field_label=label><crate::admin_textarea::AdminTextarea
                                    admin_input_name=spec.name()
                                    required=bool::from(required)
                                    disabled=bool::from(admin_setting_disabled)
                                    option=value
                                /></crate::admin_field::AdminField>
                            })
                        }
                    }
        }).collect::<Vec<_>>()}
    }
}
