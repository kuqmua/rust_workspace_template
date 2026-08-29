use super::domain_types::AdminPermissionSummary;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminPermissionsPage {
    #[contract_struct_api(into, slice = AdminPermissionSummary)]
    items: crate::domain_types::AdminPermissionSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::domain_types::AdminPageTotal,
}
