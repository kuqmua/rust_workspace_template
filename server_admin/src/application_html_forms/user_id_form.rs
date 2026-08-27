#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UserIdForm {
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
    pub(super) confirmation: server_admin_contract::domain_types::AdminBool,
}
