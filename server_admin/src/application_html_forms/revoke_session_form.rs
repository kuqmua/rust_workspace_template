#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::domain_types::auth::html) struct RevokeSessionForm {
    pub(in crate::domain_types::auth::html) session_id:
        server_admin_contract::domain_types::AdminSessionIdentifier,
    pub(in crate::domain_types::auth::html) confirmation:
        server_admin_contract::domain_types::AdminBool,
}
