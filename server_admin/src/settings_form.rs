use crate::AdminHtmlFormText;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SettingsForm {
    pub(crate) default_admin_route: server_admin_contract::domain_types::AdminDefaultRoute,
    pub(crate) main_logo: AdminHtmlFormText,
    pub(crate) organization_contacts: AdminHtmlFormText,
    pub(crate) organization_name: AdminHtmlFormText,
    pub(crate) primary_color: AdminHtmlFormText,
    pub(crate) site_name: server_admin_contract::domain_types::AdminSiteName,
    pub(crate) support_url: AdminHtmlFormText,
    pub(crate) tab_title: AdminHtmlFormText,
}
