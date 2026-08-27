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
    pub(super) default_admin_route: super::super::AdminDefaultRoute,
    #[contract_struct_api(option_borrow)]
    pub(super) main_logo: Option<super::super::AdminMainLogo>,
    #[contract_struct_api(option_borrow)]
    organization_contacts: Option<super::super::AdminOrganizationContacts>,
    #[contract_struct_api(option_borrow)]
    organization_name: Option<super::super::AdminOrganizationName>,
    #[contract_struct_api(option_borrow)]
    pub(super) primary_color: Option<super::super::AdminPrimaryColor>,
    #[contract_struct_api(borrow)]
    pub(super) site_name: super::super::AdminSiteName,
    #[contract_struct_api(option_borrow)]
    pub(super) support_url: Option<super::super::AdminSupportUrl>,
    #[contract_struct_api(option_borrow)]
    pub(super) tab_title: Option<super::super::AdminTabTitle>,
}
