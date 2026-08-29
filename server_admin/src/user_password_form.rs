#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UserPasswordForm {
    pub(crate) password: server_admin_contract::admin_new_password::AdminNewPassword,
    pub(crate) user_id: server_admin_contract::admin_user_id::AdminUserId,
}
