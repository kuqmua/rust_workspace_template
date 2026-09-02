#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct RoleIdForm {
    role_id: server_admin_contract::admin_role_id::AdminRoleId,
    confirmation: server_admin_contract::admin_bool::AdminBool,
}
