#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct ChangePasswordForm {
    current_password: server_admin_contract::admin_password::AdminPassword,
    new_password: server_admin_contract::admin_new_password::AdminNewPassword,
}
