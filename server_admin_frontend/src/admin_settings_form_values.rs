#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(crate) struct AdminSettingsFormValues(
    [super::admin_setting_input_value::AdminSettingInputValue;
        server_admin_contract::admin_setting::AdminSetting::COUNT],
);
impl From<&server_admin_contract::admin_settings_view::AdminSettingsView>
    for AdminSettingsFormValues
{
    fn from(value: &server_admin_contract::admin_settings_view::AdminSettingsView) -> Self {
        fn optional<Value>(
            value: Option<&Value>,
        ) -> super::admin_setting_input_value::AdminSettingInputValue
        where
            Value: AsRef<str>,
        {
            super::admin_setting_input_value::AdminSettingInputValue::from(
                value
                    .map(|item| item.as_ref().to_owned())
                    .unwrap_or_default()
                    .into_boxed_str(),
            )
        }
        Self(
            server_admin_contract::admin_setting::AdminSetting::ALL.map(|setting| match setting {
                server_admin_contract::admin_setting::AdminSetting::DefaultRoute => {
                    super::admin_setting_input_value::AdminSettingInputValue::from(
                        value
                            .default_admin_route()
                            .as_ref()
                            .to_owned()
                            .into_boxed_str(),
                    )
                }
                server_admin_contract::admin_setting::AdminSetting::MainLogo => {
                    optional(value.main_logo())
                }
                server_admin_contract::admin_setting::AdminSetting::OrganizationContacts => {
                    optional(value.organization_contacts())
                }
                server_admin_contract::admin_setting::AdminSetting::OrganizationName => {
                    optional(value.organization_name())
                }
                server_admin_contract::admin_setting::AdminSetting::PrimaryColor => {
                    optional(value.primary_color())
                }
                server_admin_contract::admin_setting::AdminSetting::SiteName => {
                    super::admin_setting_input_value::AdminSettingInputValue::from(
                        value.site_name().as_ref().to_owned().into_boxed_str(),
                    )
                }
                server_admin_contract::admin_setting::AdminSetting::SupportUrl => {
                    optional(value.support_url())
                }
                server_admin_contract::admin_setting::AdminSetting::TabTitle => {
                    optional(value.tab_title())
                }
            }),
        )
    }
}
impl AdminSettingsFormValues {
    pub(crate) const fn get(
        &self,
        setting: server_admin_contract::admin_setting::AdminSetting,
    ) -> &super::admin_setting_input_value::AdminSettingInputValue {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        &self.0[setting.index()]
    }
}
