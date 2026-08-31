#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct RevokeSessionForm {
    session_id: server_admin_contract::admin_session_identifier::AdminSessionIdentifier,
    confirmation: server_admin_contract::admin_bool::AdminBool,
}
