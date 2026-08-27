use super::AdminHtmlFormText;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SettingsForm {
    pub(super) default_admin_route: server_admin_contract::domain_types::AdminDefaultRoute,
    pub(super) main_logo: AdminHtmlFormText,
    pub(super) organization_contacts: AdminHtmlFormText,
    pub(super) organization_name: AdminHtmlFormText,
    pub(super) primary_color: AdminHtmlFormText,
    pub(super) site_name: server_admin_contract::domain_types::AdminSiteName,
    pub(super) support_url: AdminHtmlFormText,
    pub(super) tab_title: AdminHtmlFormText,
}
