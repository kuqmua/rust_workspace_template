#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChangePasswordForm {
    pub(crate) current_password: server_admin_contract::domain_types::AdminPassword,
    pub(crate) new_password: server_admin_contract::domain_types::AdminNewPassword,
}
