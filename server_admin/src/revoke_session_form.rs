#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RevokeSessionForm {
    pub(crate) session_id: server_admin_contract::domain_types::AdminSessionIdentifier,
    pub(crate) confirmation: server_admin_contract::domain_types::AdminBool,
}
