#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_frontend_contract_derive_unit_enum_catalog::UnitEnumCatalog,
    proc_macro_frontend_contract_derive_unit_enum_index::UnitEnumIndex,
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
                constants_str::ADMIN_UI_DEFAULT_ROUTE,
                constants_str::VALUE_ACD40F02,
                crate::admin_setting_optionality::AdminSettingOptionality::Required,
            ),
            Self::SiteName => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::ADMIN_UI_SITE_NAME,
                constants_str::VALUE_7C6A6719,
                crate::admin_setting_optionality::AdminSettingOptionality::Required,
            ),
            Self::TabTitle => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::ADMIN_UI_TAB_TITLE,
                constants_str::VALUE_FD69A71C,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::TabTitle,
                ),
            ),
            Self::OrganizationName => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::ADMIN_UI_ORGANIZATION,
                constants_str::VALUE_C41F289C,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::OrganizationName,
                ),
            ),
            Self::OrganizationContacts => (
                crate::admin_setting_input_kind::AdminSettingInputKind::TextArea,
                constants_str::ADMIN_UI_ORGANIZATION_CONTACTS,
                constants_str::VALUE_C33009C5,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::OrganizationContacts,
                ),
            ),
            Self::SupportUrl => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Url,
                constants_str::ADMIN_UI_SUPPORT_URL,
                constants_str::VALUE_9B284285,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::SupportUrl,
                ),
            ),
            Self::PrimaryColor => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Text,
                constants_str::ADMIN_UI_PRIMARY_COLOR,
                constants_str::VALUE_EAFDE0B2,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::PrimaryColor,
                ),
            ),
            Self::MainLogo => (
                crate::admin_setting_input_kind::AdminSettingInputKind::Url,
                constants_str::ADMIN_UI_MAIN_LOGO_URL,
                constants_str::VALUE_304B098A,
                crate::admin_setting_optionality::AdminSettingOptionality::Clearable(
                    crate::admin_optional_setting::AdminOptionalSetting::MainLogo,
                ),
            ),
        };
        crate::admin_setting_spec::AdminSettingSpec::new(
            crate::admin_setting_label::AdminSettingLabel::from(label),
            crate::admin_setting_name::AdminSettingName::from(name),
            input_kind,
            optionality,
        )
    }
}
