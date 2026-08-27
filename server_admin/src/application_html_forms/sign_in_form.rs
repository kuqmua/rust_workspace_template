#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SignInForm {
    pub(super) login: server_admin_contract::domain_types::AdminLogin,
    pub(super) password: server_admin_contract::domain_types::AdminPassword,
}
