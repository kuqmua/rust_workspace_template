#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CreateUserForm {
    pub(super) display_name: server_admin_contract::domain_types::AdminDisplayName,
    pub(super) login: server_admin_contract::domain_types::AdminLogin,
    pub(super) password: server_admin_contract::domain_types::AdminNewPassword,
}
