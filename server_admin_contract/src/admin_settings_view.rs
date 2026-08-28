#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSettingsView {
    #[contract_struct_api(borrow)]
    pub(super) default_admin_route: crate::AdminDefaultRoute,
    #[contract_struct_api(option_borrow)]
    pub(super) main_logo: Option<crate::AdminMainLogo>,
    #[contract_struct_api(option_borrow)]
    organization_contacts: Option<crate::AdminOrganizationContacts>,
    #[contract_struct_api(option_borrow)]
    organization_name: Option<crate::AdminOrganizationName>,
    #[contract_struct_api(option_borrow)]
    pub(super) primary_color: Option<crate::AdminPrimaryColor>,
    #[contract_struct_api(borrow)]
    pub(super) site_name: crate::AdminSiteName,
    #[contract_struct_api(option_borrow)]
    pub(super) support_url: Option<crate::AdminSupportUrl>,
    #[contract_struct_api(option_borrow)]
    pub(super) tab_title: Option<crate::AdminTabTitle>,
}
