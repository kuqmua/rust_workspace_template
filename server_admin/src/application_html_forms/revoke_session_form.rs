#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RevokeSessionForm {
    pub(super) session_id: server_admin_contract::domain_types::AdminSessionIdentifier,
    pub(super) confirmation: server_admin_contract::domain_types::AdminBool,
}
