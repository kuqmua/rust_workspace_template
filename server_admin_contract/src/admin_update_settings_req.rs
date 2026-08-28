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
    clear: crate::AdminOptionalSettings,
    default_admin_route: Option<crate::AdminDefaultRoute>,
    main_logo: Option<crate::AdminMainLogo>,
    organization_contacts: Option<crate::AdminOrganizationContacts>,
    organization_name: Option<crate::AdminOrganizationName>,
    primary_color: Option<crate::AdminPrimaryColor>,
    site_name: Option<crate::AdminSiteName>,
    support_url: Option<crate::AdminSupportUrl>,
    tab_title: Option<crate::AdminTabTitle>,
}

impl AdminUpdateSettingsReq {
    #[must_use]
    pub const fn new(
        default_admin_route: Option<crate::AdminDefaultRoute>,
        main_logo: Option<crate::AdminMainLogo>,
        organization_contacts: Option<crate::AdminOrganizationContacts>,
        organization_name: Option<crate::AdminOrganizationName>,
        primary_color: Option<crate::AdminPrimaryColor>,
        site_name: Option<crate::AdminSiteName>,
        support_url: Option<crate::AdminSupportUrl>,
        tab_title: Option<crate::AdminTabTitle>,
        clear: crate::AdminOptionalSettings,
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
        Option<crate::AdminDefaultRoute>,
        Option<crate::AdminMainLogo>,
        Option<crate::AdminOrganizationContacts>,
        Option<crate::AdminOrganizationName>,
        Option<crate::AdminPrimaryColor>,
        Option<crate::AdminSiteName>,
        Option<crate::AdminSupportUrl>,
        Option<crate::AdminTabTitle>,
        crate::AdminOptionalSettings,
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
    pub fn has_fields(&self) -> crate::AdminBool {
        crate::AdminBool::from(
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
    pub fn is_valid(&self) -> crate::AdminBool {
        let unique = self
            .clear
            .as_ref()
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        crate::AdminBool::from(
            unique.len() == self.clear.as_ref().len()
                && self.clear.as_ref().len() <= super::AdminOptionalSetting::ALL.len()
                && !(self.main_logo.is_some()
                    && unique.contains(&super::AdminOptionalSetting::MainLogo))
                && !(self.organization_contacts.is_some()
                    && unique.contains(&super::AdminOptionalSetting::OrganizationContacts))
                && !(self.organization_name.is_some()
                    && unique.contains(&super::AdminOptionalSetting::OrganizationName))
                && !(self.primary_color.is_some()
                    && unique.contains(&super::AdminOptionalSetting::PrimaryColor))
                && !(self.support_url.is_some()
                    && unique.contains(&super::AdminOptionalSetting::SupportUrl))
                && !(self.tab_title.is_some()
                    && unique.contains(&super::AdminOptionalSetting::TabTitle)),
        )
    }
}
