#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct CreateRoleForm {
    name: server_admin_contract::admin_role_name::AdminRoleName,
}
