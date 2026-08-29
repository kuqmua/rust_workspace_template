#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SettingsForm {
    pub(crate) default_admin_route: server_admin_contract::admin_default_route::AdminDefaultRoute,
    pub(crate) main_logo: crate::admin_html_form_text::AdminHtmlFormText,
    pub(crate) organization_contacts: crate::admin_html_form_text::AdminHtmlFormText,
    pub(crate) organization_name: crate::admin_html_form_text::AdminHtmlFormText,
    pub(crate) primary_color: crate::admin_html_form_text::AdminHtmlFormText,
    pub(crate) site_name: server_admin_contract::admin_site_name::AdminSiteName,
    pub(crate) support_url: crate::admin_html_form_text::AdminHtmlFormText,
    pub(crate) tab_title: crate::admin_html_form_text::AdminHtmlFormText,
}
