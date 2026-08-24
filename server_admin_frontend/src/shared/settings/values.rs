#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the settings value collection keeps its conversion adjacent to indexed access"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::AsRefStr, newtype::FromInner,
)]
pub(crate) struct AdminSettingInputValue(Box<str>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
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
