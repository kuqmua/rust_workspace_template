use super::{AdminPermissionSummary, AdminRoleSummary};

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
pub struct AdminRolesPage {
    #[contract_struct_api(into, slice = AdminRoleSummary)]
    items: crate::domain_types::AdminRoleSummaries,
    #[contract_struct_api(slice = AdminPermissionSummary)]
    permissions: crate::domain_types::AdminPermissionSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::domain_types::AdminPageTotal,
}
