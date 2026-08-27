#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UpdateRoleForm {
    pub(super) name: server_admin_contract::domain_types::AdminRoleName,
    pub(super) role_id: server_admin_contract::domain_types::AdminRoleId,
}
