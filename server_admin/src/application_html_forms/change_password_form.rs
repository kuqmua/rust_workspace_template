#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ChangePasswordForm {
    pub(super) current_password: server_admin_contract::domain_types::AdminPassword,
    pub(super) new_password: server_admin_contract::domain_types::AdminNewPassword,
}
