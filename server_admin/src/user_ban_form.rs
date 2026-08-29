#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UserBanForm {
    pub(crate) user_id: server_admin_contract::admin_user_id::AdminUserId,
    pub(crate) is_banned: server_admin_contract::admin_bool::AdminBool,
}
