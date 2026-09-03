#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_frontend_contract_derive_contract_struct_api::ContractStructApi,
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
