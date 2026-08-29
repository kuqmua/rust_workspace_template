#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChangePasswordForm {
    pub(crate) current_password: server_admin_contract::admin_password::AdminPassword,
    pub(crate) new_password: server_admin_contract::admin_new_password::AdminNewPassword,
}
