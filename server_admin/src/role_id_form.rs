#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RoleIdForm {
    pub(crate) role_id: server_admin_contract::domain_types::AdminRoleId,
    pub(crate) confirmation: server_admin_contract::domain_types::AdminBool,
}
