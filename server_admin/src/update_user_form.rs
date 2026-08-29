#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UpdateUserForm {
    pub(crate) display_name: server_admin_contract::admin_display_name::AdminDisplayName,
    pub(crate) login: server_admin_contract::admin_login::AdminLogin,
    pub(crate) user_id: server_admin_contract::admin_user_id::AdminUserId,
}
