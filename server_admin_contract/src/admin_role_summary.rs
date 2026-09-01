#[derive(generate_accessor::Getters)]
#[getters(bare)]
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
#[optimal_memory_layout(skip)]
pub struct AdminRoleSummary {
    #[getters(skip)]
    #[contract_struct_api(copy_ref)]
    id: crate::admin_role_id::AdminRoleId,
    #[getters(skip)]
    #[contract_struct_api(copy_ref)]
    is_system: crate::admin_bool::AdminBool,
    name: crate::admin_role_name::AdminRoleName,
    #[serde(default)]
    #[getters(skip)]
    #[contract_struct_api(slice = crate::admin_permission_id::AdminPermissionId)]
    permission_ids: crate::admin_permission_ids::AdminPermissionIds,
}
