#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateRoleForm {
    pub(crate) name: server_admin_contract::domain_types::AdminRoleName,
    pub(crate) role_id: server_admin_contract::domain_types::AdminRoleId,
}
