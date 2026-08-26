#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSettingsView {
    #[contract_struct_api(borrow)]
    default_admin_route: super::AdminDefaultRoute,
    #[contract_struct_api(option_borrow)]
    main_logo: Option<super::AdminMainLogo>,
    #[contract_struct_api(option_borrow)]
    organization_contacts: Option<super::AdminOrganizationContacts>,
    #[contract_struct_api(option_borrow)]
    organization_name: Option<super::AdminOrganizationName>,
    #[contract_struct_api(option_borrow)]
    primary_color: Option<super::AdminPrimaryColor>,
    #[contract_struct_api(borrow)]
    site_name: super::AdminSiteName,
    #[contract_struct_api(option_borrow)]
    support_url: Option<super::AdminSupportUrl>,
    #[contract_struct_api(option_borrow)]
    tab_title: Option<super::AdminTabTitle>,
}
#[cfg(test)]
#[path = "domain_types_settings_tests.rs"]
mod tests;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminBrandingView {
    default_admin_route: super::AdminDefaultRoute,
    main_logo: Option<super::AdminMainLogo>,
    primary_color: Option<super::AdminPrimaryColor>,
    site_name: super::AdminSiteName,
    support_url: Option<super::AdminSupportUrl>,
    tab_title: Option<super::AdminTabTitle>,
}
impl AdminBrandingView {
    #[must_use]
    pub fn from_settings(value: &AdminSettingsView) -> Self {
        Self {
            default_admin_route: value.default_admin_route.clone(),
            main_logo: value.main_logo.clone(),
            primary_color: value.primary_color.clone(),
            site_name: value.site_name.clone(),
            support_url: value.support_url.clone(),
            tab_title: value.tab_title.clone(),
        }
    }
    #[must_use]
    pub const fn default_admin_route(&self) -> &super::AdminDefaultRoute {
        &self.default_admin_route
    }
    #[must_use]
    pub const fn main_logo(&self) -> Option<&super::AdminMainLogo> {
        self.main_logo.as_ref()
    }
    #[must_use]
    pub const fn primary_color(&self) -> Option<&super::AdminPrimaryColor> {
        self.primary_color.as_ref()
    }
    #[must_use]
    pub const fn site_name(&self) -> &super::AdminSiteName {
        &self.site_name
    }
    #[must_use]
    pub const fn support_url(&self) -> Option<&super::AdminSupportUrl> {
        self.support_url.as_ref()
    }
    #[must_use]
    pub const fn tab_title(&self) -> Option<&super::AdminTabTitle> {
        self.tab_title.as_ref()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateSettingsReq {
    #[schema(max_items = 6)]
    clear: super::AdminOptionalSettings,
    default_admin_route: Option<super::AdminDefaultRoute>,
    main_logo: Option<super::AdminMainLogo>,
    organization_contacts: Option<super::AdminOrganizationContacts>,
    organization_name: Option<super::AdminOrganizationName>,
    primary_color: Option<super::AdminPrimaryColor>,
    site_name: Option<super::AdminSiteName>,
    support_url: Option<super::AdminSupportUrl>,
    tab_title: Option<super::AdminTabTitle>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminSettingInputKind {
    Text,
    TextArea,
    Url,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub struct AdminSettingLabel(&'static str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub struct AdminSettingName(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminSettingSpec {
    label: AdminSettingLabel,
    name: AdminSettingName,
    input_kind: AdminSettingInputKind,
    optionality: AdminSettingOptionality,
}
impl AdminSettingSpec {
    #[must_use]
    pub const fn input_kind(self) -> AdminSettingInputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(self) -> AdminSettingLabel {
        self.label
    }
    #[must_use]
    pub const fn name(self) -> AdminSettingName {
        self.name
    }
    #[must_use]
    pub const fn optionality(self) -> AdminSettingOptionality {
        self.optionality
    }
    #[must_use]
    pub fn required(self) -> super::AdminBool {
        super::AdminBool::from(matches!(
            self.optionality,
            AdminSettingOptionality::Required
        ))
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminSettingOptionality {
    Clearable(AdminOptionalSetting),
    Required,
}
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    frontend_contract::domain_types::UnitEnumCatalog,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminOptionalSetting {
    TabTitle,
    OrganizationName,
    OrganizationContacts,
    SupportUrl,
    PrimaryColor,
    MainLogo,
}
impl AdminSetting {
    #[must_use]
    pub fn spec(self) -> AdminSettingSpec {
        match self {
            Self::DefaultRoute => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from(constants_str::VALUE_64B5E1B6),
                name: AdminSettingName::from(constants_str::VALUE_ACD40F02),
                optionality: AdminSettingOptionality::Required,
            },
            Self::SiteName => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from(constants_str::VALUE_E98AF105),
                name: AdminSettingName::from(constants_str::VALUE_7C6A6719),
                optionality: AdminSettingOptionality::Required,
            },
            Self::TabTitle => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from(constants_str::VALUE_9A74868F),
                name: AdminSettingName::from(constants_str::VALUE_FD69A71C),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::TabTitle),
            },
            Self::OrganizationName => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from(constants_str::VALUE_D764D425),
                name: AdminSettingName::from(constants_str::VALUE_C41F289C),
                optionality: AdminSettingOptionality::Clearable(
                    AdminOptionalSetting::OrganizationName,
                ),
            },
            Self::OrganizationContacts => AdminSettingSpec {
                input_kind: AdminSettingInputKind::TextArea,
                label: AdminSettingLabel::from(constants_str::VALUE_34CD6225),
                name: AdminSettingName::from(constants_str::VALUE_C33009C5),
                optionality: AdminSettingOptionality::Clearable(
                    AdminOptionalSetting::OrganizationContacts,
                ),
            },
            Self::SupportUrl => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Url,
                label: AdminSettingLabel::from(constants_str::VALUE_6CCA2FBA),
                name: AdminSettingName::from(constants_str::VALUE_9B284285),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::SupportUrl),
            },
            Self::PrimaryColor => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Text,
                label: AdminSettingLabel::from(constants_str::VALUE_2B03958C),
                name: AdminSettingName::from(constants_str::VALUE_EAFDE0B2),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::PrimaryColor),
            },
            Self::MainLogo => AdminSettingSpec {
                input_kind: AdminSettingInputKind::Url,
                label: AdminSettingLabel::from(constants_str::VALUE_4AE21E86),
                name: AdminSettingName::from(constants_str::VALUE_304B098A),
                optionality: AdminSettingOptionality::Clearable(AdminOptionalSetting::MainLogo),
            },
        }
    }
}
impl AdminUpdateSettingsReq {
    #[must_use]
    pub const fn new(
        default_admin_route: Option<super::AdminDefaultRoute>,
        main_logo: Option<super::AdminMainLogo>,
        organization_contacts: Option<super::AdminOrganizationContacts>,
        organization_name: Option<super::AdminOrganizationName>,
        primary_color: Option<super::AdminPrimaryColor>,
        site_name: Option<super::AdminSiteName>,
        support_url: Option<super::AdminSupportUrl>,
        tab_title: Option<super::AdminTabTitle>,
        clear: super::AdminOptionalSettings,
    ) -> Self {
        Self {
            clear,
            default_admin_route,
            main_logo,
            organization_contacts,
            organization_name,
            primary_color,
            site_name,
            support_url,
            tab_title,
        }
    }
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Option<super::AdminDefaultRoute>,
        Option<super::AdminMainLogo>,
        Option<super::AdminOrganizationContacts>,
        Option<super::AdminOrganizationName>,
        Option<super::AdminPrimaryColor>,
        Option<super::AdminSiteName>,
        Option<super::AdminSupportUrl>,
        Option<super::AdminTabTitle>,
        super::AdminOptionalSettings,
    ) {
        (
            self.default_admin_route,
            self.main_logo,
            self.organization_contacts,
            self.organization_name,
            self.primary_color,
            self.site_name,
            self.support_url,
            self.tab_title,
            self.clear,
        )
    }
    #[must_use]
    pub fn has_fields(&self) -> super::AdminBool {
        super::AdminBool::from(
            self.default_admin_route.is_some()
                || self.main_logo.is_some()
                || self.organization_contacts.is_some()
                || self.organization_name.is_some()
                || self.primary_color.is_some()
                || self.site_name.is_some()
                || self.support_url.is_some()
                || self.tab_title.is_some()
                || !self.clear.as_ref().is_empty(),
        )
    }
    #[must_use]
    pub fn is_valid(&self) -> super::AdminBool {
        let unique = self
            .clear
            .as_ref()
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        super::AdminBool::from(
            unique.len() == self.clear.as_ref().len()
                && self.clear.as_ref().len() <= AdminOptionalSetting::ALL.len()
                && !(self.main_logo.is_some() && unique.contains(&AdminOptionalSetting::MainLogo))
                && !(self.organization_contacts.is_some()
                    && unique.contains(&AdminOptionalSetting::OrganizationContacts))
                && !(self.organization_name.is_some()
                    && unique.contains(&AdminOptionalSetting::OrganizationName))
                && !(self.primary_color.is_some()
                    && unique.contains(&AdminOptionalSetting::PrimaryColor))
                && !(self.support_url.is_some()
                    && unique.contains(&AdminOptionalSetting::SupportUrl))
                && !(self.tab_title.is_some() && unique.contains(&AdminOptionalSetting::TabTitle)),
        )
    }
}
