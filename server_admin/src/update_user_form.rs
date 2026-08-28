#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateUserForm {
    pub(crate) display_name: server_admin_contract::domain_types::AdminDisplayName,
    pub(crate) login: server_admin_contract::domain_types::AdminLogin,
    pub(crate) user_id: server_admin_contract::domain_types::AdminUserId,
}
