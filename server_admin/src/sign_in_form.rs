#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct SignInForm {
    login: server_admin_contract::admin_login::AdminLogin,
    password: server_admin_contract::admin_password::AdminPassword,
}
