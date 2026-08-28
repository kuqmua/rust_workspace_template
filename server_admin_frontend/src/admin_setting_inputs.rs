#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(crate) fn admin_setting_inputs(
    signals: crate::admin_settings_form_signals::AdminSettingsFormSignals,
    disabled: super::AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {server_admin_contract::domain_types::AdminSetting::ALL.into_iter().map(|setting| {
            {
                let field = setting;
                let value = signals.get(setting);
                    let spec = field.spec();
                    let required = super::AdminSettingRequired::from(bool::from(spec.required()));
                    match spec.input_kind() {
                        server_admin_contract::domain_types::AdminSettingInputKind::Text
                        | server_admin_contract::domain_types::AdminSettingInputKind::Url => {
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
                            leptos::prelude::IntoAny::into_any(leptos::view! {
                                <crate::domain_types::with_owner::field::AdminField label=label><crate::domain_types::with_owner::input::AdminInput
                                    name=spec.name()
                                    kind=input_kind
                                    required=bool::from(required)
                                    disabled=bool::from(disabled)
                                    bind_value=value
                                /></crate::domain_types::with_owner::field::AdminField>
                            })
                        }
                        server_admin_contract::domain_types::AdminSettingInputKind::TextArea => {
                            let label = spec.label().as_ref().to_owned();
                            leptos::prelude::IntoAny::into_any(leptos::view! {
                                <crate::domain_types::with_owner::field::AdminField label=label><crate::domain_types::with_owner::admin_textarea::AdminTextarea
                                    name=spec.name()
                                    required=bool::from(required)
                                    disabled=bool::from(disabled)
                                    bind_value=value
                                /></crate::domain_types::with_owner::field::AdminField>
                            })
                        }
                    }
            }
        }).collect::<Vec<_>>()}
    }
}
