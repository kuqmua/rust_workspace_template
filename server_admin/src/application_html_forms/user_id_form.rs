#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::domain_types::auth::html) struct UserIdForm {
    pub(in crate::domain_types::auth::html) user_id:
        server_admin_contract::domain_types::AdminUserId,
    pub(in crate::domain_types::auth::html) confirmation:
        server_admin_contract::domain_types::AdminBool,
}
