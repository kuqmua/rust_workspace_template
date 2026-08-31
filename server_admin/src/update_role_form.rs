#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct UpdateRoleForm {
    name: server_admin_contract::admin_role_name::AdminRoleName,
    role_id: server_admin_contract::admin_role_id::AdminRoleId,
}
