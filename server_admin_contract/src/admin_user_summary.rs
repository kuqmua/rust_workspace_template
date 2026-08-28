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
#[optimal_memory_layout(skip)]
pub struct AdminUserSummary {
    #[contract_struct_api(borrow)]
    display_name: crate::domain_types::AdminDisplayName,
    #[contract_struct_api(copy_ref)]
    id: crate::domain_types::AdminUserId,
    #[contract_struct_api(copy_ref)]
    is_banned: crate::domain_types::AdminBool,
    #[contract_struct_api(borrow)]
    login: crate::domain_types::AdminLogin,
    #[serde(default)]
    #[contract_struct_api(slice = crate::domain_types::AdminRoleId)]
    role_ids: crate::domain_types::AdminRoleIds,
}
