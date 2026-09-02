#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct CreateUserForm {
    display_name: server_admin_contract::admin_display_name::AdminDisplayName,
    login: server_admin_contract::admin_login::AdminLogin,
    password: server_admin_contract::admin_new_password::AdminNewPassword,
}
