#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SignInForm {
    pub(crate) login: server_admin_contract::admin_login::AdminLogin,
    pub(crate) password: server_admin_contract::admin_password::AdminPassword,
}
