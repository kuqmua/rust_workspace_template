#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct UserPasswordForm {
    password: server_admin_contract::admin_new_password::AdminNewPassword,
    user_id: server_admin_contract::admin_user_id::AdminUserId,
}
