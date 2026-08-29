#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RoleIdForm {
    pub(crate) role_id: server_admin_contract::admin_role_id::AdminRoleId,
    pub(crate) confirmation: server_admin_contract::admin_bool::AdminBool,
}
