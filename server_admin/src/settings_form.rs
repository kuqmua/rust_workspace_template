use super::AdminHtmlFormText;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::domain_types::auth::html) struct SettingsForm {
    pub(in crate::domain_types::auth::html) default_admin_route:
        server_admin_contract::domain_types::AdminDefaultRoute,
    pub(in crate::domain_types::auth::html) main_logo: AdminHtmlFormText,
    pub(in crate::domain_types::auth::html) organization_contacts: AdminHtmlFormText,
    pub(in crate::domain_types::auth::html) organization_name: AdminHtmlFormText,
    pub(in crate::domain_types::auth::html) primary_color: AdminHtmlFormText,
    pub(in crate::domain_types::auth::html) site_name:
        server_admin_contract::domain_types::AdminSiteName,
    pub(in crate::domain_types::auth::html) support_url: AdminHtmlFormText,
    pub(in crate::domain_types::auth::html) tab_title: AdminHtmlFormText,
}
