#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminBrandingView {
    default_admin_route: crate::AdminDefaultRoute,
    main_logo: Option<crate::AdminMainLogo>,
    primary_color: Option<crate::AdminPrimaryColor>,
    site_name: crate::AdminSiteName,
    support_url: Option<crate::AdminSupportUrl>,
    tab_title: Option<crate::AdminTabTitle>,
}

impl AdminBrandingView {
    #[must_use]
    pub fn from_settings(value: &super::AdminSettingsView) -> Self {
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
    pub const fn default_admin_route(&self) -> &crate::AdminDefaultRoute {
        &self.default_admin_route
    }
    #[must_use]
    pub const fn main_logo(&self) -> Option<&crate::AdminMainLogo> {
        self.main_logo.as_ref()
    }
    #[must_use]
    pub const fn primary_color(&self) -> Option<&crate::AdminPrimaryColor> {
        self.primary_color.as_ref()
    }
    #[must_use]
    pub const fn site_name(&self) -> &crate::AdminSiteName {
        &self.site_name
    }
    #[must_use]
    pub const fn support_url(&self) -> Option<&crate::AdminSupportUrl> {
        self.support_url.as_ref()
    }
    #[must_use]
    pub const fn tab_title(&self) -> Option<&crate::AdminTabTitle> {
        self.tab_title.as_ref()
    }
}
