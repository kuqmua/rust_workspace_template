#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    frontend_contract_macros::UnitEnumCatalog,
    frontend_contract_macros::UnitEnumIndex,
)]
pub enum AdminSetting {
    DefaultRoute,
    SiteName,
    TabTitle,
    OrganizationName,
    OrganizationContacts,
    SupportUrl,
    PrimaryColor,
    MainLogo,
}

impl AdminSetting {
    #[must_use]
    pub fn spec(self) -> crate::admin_setting_spec::AdminSettingSpec {
        let (input_kind, label, name, optionality) = match self {
            Self::DefaultRoute => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::test_fixtures::VALUE_64B5E1B6,
                constants_str::test_fixtures::VALUE_ACD40F02,
                crate::admin_setting_optionality::AdminSettingOptionality::Required,
            ),
            Self::SiteName => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::test_fixtures::VALUE_E98AF105,
                constants_str::test_fixtures::VALUE_7C6A6719,
                crate::admin_setting_optionality::AdminSettingOptionality::Required,
            ),
            Self::TabTitle => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::test_fixtures::VALUE_9A74868F,
                constants_str::test_fixtures::VALUE_FD69A71C,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::TabTitle,
                ),
            ),
            Self::OrganizationName => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::test_fixtures::VALUE_D764D425,
                constants_str::test_fixtures::VALUE_C41F289C,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::OrganizationName,
                ),
            ),
            Self::OrganizationContacts => (
                crate::admin_setting_input_kind::AdminSettingInputKind::TextArea,
                constants_str::test_fixtures::VALUE_34CD6225,
                constants_str::test_fixtures::VALUE_C33009C5,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::OrganizationContacts,
                ),
            ),
            Self::SupportUrl => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Url,
                constants_str::test_fixtures::VALUE_6CCA2FBA,
                constants_str::test_fixtures::VALUE_9B284285,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::SupportUrl,
                ),
            ),
            Self::PrimaryColor => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::test_fixtures::VALUE_2B03958C,
                constants_str::test_fixtures::VALUE_EAFDE0B2,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::PrimaryColor,
                ),
            ),
            Self::MainLogo => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Url,
                constants_str::test_fixtures::VALUE_4AE21E86,
                constants_str::test_fixtures::VALUE_304B098A,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::MainLogo,
                ),
            ),
        };
        crate::admin_setting_spec::AdminSettingSpec {
            input_kind,
            label: crate::admin_setting_label::AdminSettingLabel::from(label),
            name: crate::admin_setting_name::AdminSettingName::from(name),
            optionality,
        }
    }
}
