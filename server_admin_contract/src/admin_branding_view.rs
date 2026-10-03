#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
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
    pub fn from_settings(
        admin_settings_view: &crate::admin_settings_view::AdminSettingsView,
    ) -> Self {
        Self {
            default_admin_route: admin_settings_view.default_admin_route().clone(),
            main_logo: admin_settings_view.main_logo().cloned(),
            primary_color: admin_settings_view.primary_color().cloned(),
            site_name: admin_settings_view.site_name().clone(),
            support_url: admin_settings_view.support_url().cloned(),
            tab_title: admin_settings_view.tab_title().cloned(),
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_branding_projection_preserves_every_optional_field_combination() {
        assert!((0usize..16usize).all(|mask| {
            let expected = serde_json::json!({
                (stringify!(default_admin_route)): crate::admin_frontend_path::AdminFrontendPath::Users.get(),
                (stringify!(main_logo)): (mask & 1usize != 0usize).then_some(constants_str::ADMIN_DEFAULT_MAIN_LOGO),
                (stringify!(primary_color)): (mask & 2usize != 0usize).then_some(constants_str::PRIMARY_COLOR_DEFAULT),
                (stringify!(site_name)): constants_str::X,
                (stringify!(support_url)): (mask & 4usize != 0usize).then_some(constants_str::ADMIN_DEFAULT_SUPPORT_URL),
                (stringify!(tab_title)): (mask & 8usize != 0usize).then_some(constants_str::ADMIN),
            });
            let mut settings_wire = expected.clone();
            let extended = settings_wire.as_object_mut().is_some_and(|fields| {
                fields.insert(stringify!(organization_name).to_owned(), serde_json::json!(constants_str::LOGIN)).is_none()
                    && fields.insert(stringify!(organization_contacts).to_owned(), serde_json::json!(constants_str::ADMIN)).is_none()
            });
            extended && serde_json::from_value::<crate::admin_settings_view::AdminSettingsView>(settings_wire).is_ok_and(|settings| {
                let branding = super::AdminBrandingView::from_settings(&settings);
                branding.default_admin_route().as_ref() == settings.default_admin_route().as_ref()
                    && branding.site_name().as_ref() == settings.site_name().as_ref()
                    && branding.main_logo().map(AsRef::as_ref) == settings.main_logo().map(AsRef::as_ref)
                    && branding.primary_color().map(AsRef::as_ref) == settings.primary_color().map(AsRef::as_ref)
                    && branding.support_url().map(AsRef::as_ref) == settings.support_url().map(AsRef::as_ref)
                    && branding.tab_title().map(AsRef::as_ref) == settings.tab_title().map(AsRef::as_ref)
                    && serde_json::to_value(branding).is_ok_and(|wire| wire == expected)
            })
        }));
    }
}
