#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateRoleForm {
    pub(crate) name: server_admin_contract::admin_role_name::AdminRoleName,
    pub(crate) role_id: server_admin_contract::admin_role_id::AdminRoleId,
}
