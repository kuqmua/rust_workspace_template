#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct UpdateUserForm {
    display_name: server_admin_contract::admin_display_name::AdminDisplayName,
    login: server_admin_contract::admin_login::AdminLogin,
    user_id: server_admin_contract::admin_user_id::AdminUserId,
}
