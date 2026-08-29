#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract_macros::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSettingsView {
    #[contract_struct_api(borrow)]
    pub(super) default_admin_route: crate::admin_default_route::AdminDefaultRoute,
    #[contract_struct_api(option_borrow)]
    pub(super) main_logo: Option<crate::admin_main_logo::AdminMainLogo>,
    #[contract_struct_api(option_borrow)]
    organization_contacts: Option<crate::admin_organization_contacts::AdminOrganizationContacts>,
    #[contract_struct_api(option_borrow)]
    organization_name: Option<crate::admin_organization_name::AdminOrganizationName>,
    #[contract_struct_api(option_borrow)]
    pub(super) primary_color: Option<crate::admin_primary_color::AdminPrimaryColor>,
    #[contract_struct_api(borrow)]
    pub(super) site_name: crate::admin_site_name::AdminSiteName,
    #[contract_struct_api(option_borrow)]
    pub(super) support_url: Option<crate::admin_support_url::AdminSupportUrl>,
    #[contract_struct_api(option_borrow)]
    pub(super) tab_title: Option<crate::admin_tab_title::AdminTabTitle>,
}
