#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct SettingsForm {
    default_admin_route: server_admin_contract::admin_default_route::AdminDefaultRoute,
    main_logo: crate::admin_html_form_text::AdminHtmlFormText,
    organization_contacts: crate::admin_html_form_text::AdminHtmlFormText,
    organization_name: crate::admin_html_form_text::AdminHtmlFormText,
    primary_color: crate::admin_html_form_text::AdminHtmlFormText,
    site_name: server_admin_contract::admin_site_name::AdminSiteName,
    support_url: crate::admin_html_form_text::AdminHtmlFormText,
    tab_title: crate::admin_html_form_text::AdminHtmlFormText,
}

impl SettingsForm {
    #[allow(
        clippy::single_call_fn,
        reason = "consuming access is required to keep every form field private without cloning"
    )]
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin_contract::admin_default_route::AdminDefaultRoute,
        crate::admin_html_form_text::AdminHtmlFormText,
        crate::admin_html_form_text::AdminHtmlFormText,
        crate::admin_html_form_text::AdminHtmlFormText,
        crate::admin_html_form_text::AdminHtmlFormText,
        server_admin_contract::admin_site_name::AdminSiteName,
        crate::admin_html_form_text::AdminHtmlFormText,
        crate::admin_html_form_text::AdminHtmlFormText,
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
        )
    }
}
