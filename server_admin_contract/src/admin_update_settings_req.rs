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
    clear: crate::admin_optional_settings::AdminOptionalSettings,
    default_admin_route: Option<crate::admin_default_route::AdminDefaultRoute>,
    main_logo: Option<crate::admin_main_logo::AdminMainLogo>,
    organization_contacts: Option<crate::admin_organization_contacts::AdminOrganizationContacts>,
    organization_name: Option<crate::admin_organization_name::AdminOrganizationName>,
    primary_color: Option<crate::admin_primary_color::AdminPrimaryColor>,
    site_name: Option<crate::admin_site_name::AdminSiteName>,
    support_url: Option<crate::admin_support_url::AdminSupportUrl>,
    tab_title: Option<crate::admin_tab_title::AdminTabTitle>,
}

impl AdminUpdateSettingsReq {
    #[must_use]
    pub const fn new(
        default_admin_route: Option<crate::admin_default_route::AdminDefaultRoute>,
        main_logo: Option<crate::admin_main_logo::AdminMainLogo>,
        organization_contacts: Option<
            crate::admin_organization_contacts::AdminOrganizationContacts,
        >,
        organization_name: Option<crate::admin_organization_name::AdminOrganizationName>,
        primary_color: Option<crate::admin_primary_color::AdminPrimaryColor>,
        site_name: Option<crate::admin_site_name::AdminSiteName>,
        support_url: Option<crate::admin_support_url::AdminSupportUrl>,
        tab_title: Option<crate::admin_tab_title::AdminTabTitle>,
        clear: crate::admin_optional_settings::AdminOptionalSettings,
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
        Option<crate::admin_default_route::AdminDefaultRoute>,
        Option<crate::admin_main_logo::AdminMainLogo>,
        Option<crate::admin_organization_contacts::AdminOrganizationContacts>,
        Option<crate::admin_organization_name::AdminOrganizationName>,
        Option<crate::admin_primary_color::AdminPrimaryColor>,
        Option<crate::admin_site_name::AdminSiteName>,
        Option<crate::admin_support_url::AdminSupportUrl>,
        Option<crate::admin_tab_title::AdminTabTitle>,
        crate::admin_optional_settings::AdminOptionalSettings,
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
    pub fn has_fields(&self) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(
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
    pub fn is_valid(&self) -> crate::admin_bool::AdminBool {
        let unique = self
            .clear
            .as_ref()
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>();
        crate::admin_bool::AdminBool::from(
            unique.len() == self.clear.as_ref().len()
                && self.clear.as_ref().len()
                    <= crate::admin_optional_setting::AdminOptionalSetting::ALL.len()
                && !(self.main_logo.is_some()
                    && unique
                        .contains(&crate::admin_optional_setting::AdminOptionalSetting::MainLogo))
                && !(self.organization_contacts.is_some()
                    && unique.contains(
                        &crate::admin_optional_setting::AdminOptionalSetting::OrganizationContacts,
                    ))
                && !(self.organization_name.is_some()
                    && unique.contains(
                        &crate::admin_optional_setting::AdminOptionalSetting::OrganizationName,
                    ))
                && !(self.primary_color.is_some()
                    && unique.contains(
                        &crate::admin_optional_setting::AdminOptionalSetting::PrimaryColor,
                    ))
                && !(self.support_url.is_some()
                    && unique.contains(
                        &crate::admin_optional_setting::AdminOptionalSetting::SupportUrl,
                    ))
                && !(self.tab_title.is_some()
                    && unique
                        .contains(&crate::admin_optional_setting::AdminOptionalSetting::TabTitle)),
        )
    }
}
