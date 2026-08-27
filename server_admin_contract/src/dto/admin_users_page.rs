use super::{AdminRoleSummary, AdminUserSummary};

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
pub struct AdminUsersPage {
    #[contract_struct_api(into, slice = AdminUserSummary)]
    items: crate::domain_types::AdminUserSummaries,
    #[contract_struct_api(slice = AdminRoleSummary)]
    roles: crate::domain_types::AdminRoleSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::domain_types::AdminPageTotal,
}
