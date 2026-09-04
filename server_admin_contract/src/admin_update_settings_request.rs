#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(deny_unknown_fields)]
#[derive(proc_macro_new::New)]
pub struct AdminUpdateSettingsRequest {
    #[schema(max_items = 6)]
    #[constructor(order = 8)]
    clear: crate::admin_optional_settings::AdminOptionalSettings,
    #[constructor(order = 0)]
    default_admin_route: Option<crate::admin_default_route::AdminDefaultRoute>,
    #[constructor(order = 1)]
    main_logo: Option<crate::admin_main_logo::AdminMainLogo>,
    #[constructor(order = 2)]
    organization_contacts: Option<crate::admin_organization_contacts::AdminOrganizationContacts>,
    #[constructor(order = 3)]
    organization_name: Option<crate::admin_organization_name::AdminOrganizationName>,
    #[constructor(order = 4)]
    primary_color: Option<crate::admin_primary_color::AdminPrimaryColor>,
    #[constructor(order = 5)]
    site_name: Option<crate::admin_site_name::AdminSiteName>,
    #[constructor(order = 6)]
    support_url: Option<crate::admin_support_url::AdminSupportUrl>,
    #[constructor(order = 7)]
    tab_title: Option<crate::admin_tab_title::AdminTabTitle>,
}

impl AdminUpdateSettingsRequest {
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
