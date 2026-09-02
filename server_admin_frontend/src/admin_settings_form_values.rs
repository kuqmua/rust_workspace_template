#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(crate) struct AdminSettingsFormValues(
    [super::admin_setting_input_value::AdminSettingInputValue;
        server_admin_contract::admin_setting::AdminSetting::COUNT],
);
impl From<&server_admin_contract::admin_settings_view::AdminSettingsView>
    for AdminSettingsFormValues
{
    fn from(
        admin_settings_view: &server_admin_contract::admin_settings_view::AdminSettingsView,
    ) -> Self {
        fn optional<Value>(
            option: Option<&Value>,
        ) -> super::admin_setting_input_value::AdminSettingInputValue
        where
            Value: AsRef<str>,
        {
            super::admin_setting_input_value::AdminSettingInputValue::from(
                option
                    .map(|item| item.as_ref().to_owned())
                    .unwrap_or_default()
                    .into_boxed_str(),
            )
        }
        Self(
            server_admin_contract::admin_setting::AdminSetting::ALL.map(|setting| match setting {
                server_admin_contract::admin_setting::AdminSetting::DefaultRoute => {
                    super::admin_setting_input_value::AdminSettingInputValue::from(
                        admin_settings_view
                            .default_admin_route()
                            .as_ref()
                            .to_owned()
                            .into_boxed_str(),
                    )
                }
                server_admin_contract::admin_setting::AdminSetting::MainLogo => {
                    optional(admin_settings_view.main_logo())
                }
                server_admin_contract::admin_setting::AdminSetting::OrganizationContacts => {
                    optional(admin_settings_view.organization_contacts())
                }
                server_admin_contract::admin_setting::AdminSetting::OrganizationName => {
                    optional(admin_settings_view.organization_name())
                }
                server_admin_contract::admin_setting::AdminSetting::PrimaryColor => {
                    optional(admin_settings_view.primary_color())
                }
                server_admin_contract::admin_setting::AdminSetting::SiteName => {
                    super::admin_setting_input_value::AdminSettingInputValue::from(
                        admin_settings_view
                            .site_name()
                            .as_ref()
                            .to_owned()
                            .into_boxed_str(),
                    )
                }
                server_admin_contract::admin_setting::AdminSetting::SupportUrl => {
                    optional(admin_settings_view.support_url())
                }
                server_admin_contract::admin_setting::AdminSetting::TabTitle => {
                    optional(admin_settings_view.tab_title())
                }
            }),
        )
    }
}
impl AdminSettingsFormValues {
    pub(crate) const fn get(
        &self,
        admin_setting: server_admin_contract::admin_setting::AdminSetting,
    ) -> &super::admin_setting_input_value::AdminSettingInputValue {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        &self.0[admin_setting.index()]
    }
}
