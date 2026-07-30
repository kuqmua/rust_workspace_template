#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "shared Leptos renderers stay adjacent to their field metadata; view expansion requires attribute traits, consumes converted query values, and each target uses the shared renderer once"
)]

use leptos::prelude::{ElementChild, OnAttribute};

#[derive(Clone, Debug, newtype::AsRefStr, newtype::FromInner)]
pub(crate) struct AdminSettingInputValue(Box<str>);

#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub(crate) struct AdminSettingDisabled(bool);
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct AdminSettingRequired(bool);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct LeptosAdminSettingSignal(leptos::prelude::RwSignal<String>);
impl LeptosAdminSettingSignal {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn value(self) -> AdminSettingInputValue {
        AdminSettingInputValue::from(leptos::prelude::Get::get(&self.0).into_boxed_str())
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

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminSettingsFormSignals(
    [LeptosAdminSettingSignal; server_admin_contract::AdminSetting::COUNT],
);
impl AdminSettingsFormSignals {
    pub(crate) fn new(values: &AdminSettingsFormValues) -> Self {
        Self::from(server_admin_contract::AdminSetting::ALL.map(|setting| {
            LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.get(setting).as_ref().to_owned(),
            ))
        }))
    }
    pub(crate) const fn get(
        self,
        setting: server_admin_contract::AdminSetting,
    ) -> LeptosAdminSettingSignal {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        self.0[setting.index()]
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn optional_settings_to_clear(
        self,
    ) -> Result<
        server_admin_contract::AdminOptionalSettings,
        server_admin_contract::AdminCollectionError,
    > {
        let values = server_admin_contract::AdminSetting::ALL
            .into_iter()
            .filter_map(|setting| match setting.spec().optionality() {
                server_admin_contract::AdminSettingOptionality::Clearable(optional)
                    if self.get(setting).value().as_ref().is_empty() =>
                {
                    Some(optional)
                }
                server_admin_contract::AdminSettingOptionality::Clearable(_)
                | server_admin_contract::AdminSettingOptionality::Required => None,
            })
            .collect::<Vec<_>>();
        server_admin_contract::AdminOptionalSettings::try_from(values)
    }
}

pub(crate) fn admin_setting_inputs(
    signals: AdminSettingsFormSignals,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {server_admin_contract::AdminSetting::ALL.into_iter().map(|setting| {
            admin_setting_input(setting, signals.get(setting), disabled)
        }).collect::<Vec<_>>()}
    }
}

#[derive(Clone, Debug)]
pub(crate) struct AdminSettingsFormValues(
    [AdminSettingInputValue; server_admin_contract::AdminSetting::COUNT],
);
impl From<&server_admin_contract::AdminSettingsView> for AdminSettingsFormValues {
    fn from(value: &server_admin_contract::AdminSettingsView) -> Self {
        fn optional<Value>(value: Option<&Value>) -> AdminSettingInputValue
        where
            Value: AsRef<str>,
        {
            AdminSettingInputValue::from(
                value
                    .map(|item| item.as_ref().to_owned())
                    .unwrap_or_default()
                    .into_boxed_str(),
            )
        }
        Self(server_admin_contract::AdminSetting::ALL.map(|setting| {
            match setting {
                server_admin_contract::AdminSetting::DefaultRoute => AdminSettingInputValue::from(
                    value
                        .default_admin_route()
                        .as_ref()
                        .to_owned()
                        .into_boxed_str(),
                ),
                server_admin_contract::AdminSetting::MainLogo => optional(value.main_logo()),
                server_admin_contract::AdminSetting::OrganizationContacts => {
                    optional(value.organization_contacts())
                }
                server_admin_contract::AdminSetting::OrganizationName => {
                    optional(value.organization_name())
                }
                server_admin_contract::AdminSetting::PrimaryColor => {
                    optional(value.primary_color())
                }
                server_admin_contract::AdminSetting::SiteName => AdminSettingInputValue::from(
                    value.site_name().as_ref().to_owned().into_boxed_str(),
                ),
                server_admin_contract::AdminSetting::SupportUrl => optional(value.support_url()),
                server_admin_contract::AdminSetting::TabTitle => optional(value.tab_title()),
            }
        }))
    }
}
impl AdminSettingsFormValues {
    pub(crate) const fn get(
        &self,
        setting: server_admin_contract::AdminSetting,
    ) -> &AdminSettingInputValue {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        &self.0[setting.index()]
    }
}
