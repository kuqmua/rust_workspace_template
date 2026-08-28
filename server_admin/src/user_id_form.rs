#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UserIdForm {
    pub(crate) user_id: server_admin_contract::domain_types::AdminUserId,
    pub(crate) confirmation: server_admin_contract::domain_types::AdminBool,
}
