#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminBrandingView {
    default_admin_route: crate::admin_default_route::AdminDefaultRoute,
    #[getters(skip)]
    main_logo: Option<crate::admin_main_logo::AdminMainLogo>,
    #[getters(skip)]
    primary_color: Option<crate::admin_primary_color::AdminPrimaryColor>,
    site_name: crate::admin_site_name::AdminSiteName,
    #[getters(skip)]
    support_url: Option<crate::admin_support_url::AdminSupportUrl>,
    #[getters(skip)]
    tab_title: Option<crate::admin_tab_title::AdminTabTitle>,
}

impl AdminBrandingView {
    #[must_use]
    pub fn from_settings(value: &crate::admin_settings_view::AdminSettingsView) -> Self {
        Self {
            default_admin_route: value.default_admin_route().clone(),
            main_logo: value.main_logo().cloned(),
            primary_color: value.primary_color().cloned(),
            site_name: value.site_name().clone(),
            support_url: value.support_url().cloned(),
            tab_title: value.tab_title().cloned(),
        }
    }

    #[must_use]
    pub const fn main_logo(&self) -> Option<&crate::admin_main_logo::AdminMainLogo> {
        self.main_logo.as_ref()
    }
    #[must_use]
    pub const fn primary_color(&self) -> Option<&crate::admin_primary_color::AdminPrimaryColor> {
        self.primary_color.as_ref()
    }

    #[must_use]
    pub const fn support_url(&self) -> Option<&crate::admin_support_url::AdminSupportUrl> {
        self.support_url.as_ref()
    }
    #[must_use]
    pub const fn tab_title(&self) -> Option<&crate::admin_tab_title::AdminTabTitle> {
        self.tab_title.as_ref()
    }
}
