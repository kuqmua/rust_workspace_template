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
    clear: super::super::AdminOptionalSettings,
    default_admin_route: Option<super::super::AdminDefaultRoute>,
    main_logo: Option<super::super::AdminMainLogo>,
    organization_contacts: Option<super::super::AdminOrganizationContacts>,
    organization_name: Option<super::super::AdminOrganizationName>,
    primary_color: Option<super::super::AdminPrimaryColor>,
    site_name: Option<super::super::AdminSiteName>,
    support_url: Option<super::super::AdminSupportUrl>,
    tab_title: Option<super::super::AdminTabTitle>,
}

impl AdminUpdateSettingsReq {
    #[must_use]
    pub const fn new(
        default_admin_route: Option<super::super::AdminDefaultRoute>,
        main_logo: Option<super::super::AdminMainLogo>,
        organization_contacts: Option<super::super::AdminOrganizationContacts>,
        organization_name: Option<super::super::AdminOrganizationName>,
        primary_color: Option<super::super::AdminPrimaryColor>,
        site_name: Option<super::super::AdminSiteName>,
        support_url: Option<super::super::AdminSupportUrl>,
        tab_title: Option<super::super::AdminTabTitle>,
        clear: super::super::AdminOptionalSettings,
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
        Option<super::super::AdminDefaultRoute>,
        Option<super::super::AdminMainLogo>,
        Option<super::super::AdminOrganizationContacts>,
        Option<super::super::AdminOrganizationName>,
        Option<super::super::AdminPrimaryColor>,
        Option<super::super::AdminSiteName>,
        Option<super::super::AdminSupportUrl>,
        Option<super::super::AdminTabTitle>,
        super::super::AdminOptionalSettings,
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
    pub fn has_fields(&self) -> super::super::AdminBool {
        super::super::AdminBool::from(
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
    pub fn is_valid(&self) -> super::super::AdminBool {
        let unique = self
            .clear
            .as_ref()
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        super::super::AdminBool::from(
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
