#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RoleIdForm {
    pub(super) role_id: server_admin_contract::domain_types::AdminRoleId,
    pub(super) confirmation: server_admin_contract::domain_types::AdminBool,
}
