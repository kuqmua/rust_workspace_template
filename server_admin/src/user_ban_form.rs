#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct UserBanForm {
    user_id: server_admin_contract::admin_user_id::AdminUserId,
    is_banned: server_admin_contract::admin_bool::AdminBool,
}
