#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    frontend_contract::domain_types::UnitEnumCatalog,
    frontend_contract::domain_types::UnitEnumIndex,
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
    pub fn spec(self) -> super::AdminSettingSpec {
        let (input_kind, label, name, optionality) = match self {
            Self::DefaultRoute => (
                super::AdminSettingInputKind::Text,
                constants_str::VALUE_64B5E1B6,
                constants_str::VALUE_ACD40F02,
                super::AdminSettingOptionality::Required,
            ),
            Self::SiteName => (
                super::AdminSettingInputKind::Text,
                constants_str::VALUE_E98AF105,
                constants_str::VALUE_7C6A6719,
                super::AdminSettingOptionality::Required,
            ),
            Self::TabTitle => (
                super::AdminSettingInputKind::Text,
                constants_str::VALUE_9A74868F,
                constants_str::VALUE_FD69A71C,
                super::AdminSettingOptionality::Clearable(super::AdminOptionalSetting::TabTitle),
            ),
            Self::OrganizationName => (
                super::AdminSettingInputKind::Text,
                constants_str::VALUE_D764D425,
                constants_str::VALUE_C41F289C,
                super::AdminSettingOptionality::Clearable(
                    super::AdminOptionalSetting::OrganizationName,
                ),
            ),
            Self::OrganizationContacts => (
                super::AdminSettingInputKind::TextArea,
                constants_str::VALUE_34CD6225,
                constants_str::VALUE_C33009C5,
                super::AdminSettingOptionality::Clearable(
                    super::AdminOptionalSetting::OrganizationContacts,
                ),
            ),
            Self::SupportUrl => (
                super::AdminSettingInputKind::Url,
                constants_str::VALUE_6CCA2FBA,
                constants_str::VALUE_9B284285,
                super::AdminSettingOptionality::Clearable(super::AdminOptionalSetting::SupportUrl),
            ),
            Self::PrimaryColor => (
                super::AdminSettingInputKind::Text,
                constants_str::VALUE_2B03958C,
                constants_str::VALUE_EAFDE0B2,
                super::AdminSettingOptionality::Clearable(
                    super::AdminOptionalSetting::PrimaryColor,
                ),
            ),
            Self::MainLogo => (
                super::AdminSettingInputKind::Url,
                constants_str::VALUE_4AE21E86,
                constants_str::VALUE_304B098A,
                super::AdminSettingOptionality::Clearable(super::AdminOptionalSetting::MainLogo),
            ),
        };
        super::AdminSettingSpec {
            input_kind,
            label: super::AdminSettingLabel::from(label),
            name: super::AdminSettingName::from(name),
            optionality,
        }
    }
}
