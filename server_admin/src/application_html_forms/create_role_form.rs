#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CreateRoleForm {
    pub(super) name: server_admin_contract::domain_types::AdminRoleName,
}
