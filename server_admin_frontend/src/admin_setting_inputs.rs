pub(crate) fn admin_setting_inputs(
    signals: super::super::admin_settings_form_signals::AdminSettingsFormSignals,
    disabled: super::AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {server_admin_contract::domain_types::AdminSetting::ALL.into_iter().map(|setting| {
            super::admin_setting_input(setting, signals.get(setting), disabled)
        }).collect::<Vec<_>>()}
    }
}
