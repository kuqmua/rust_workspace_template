#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UserPasswordForm {
    pub(super) password: server_admin_contract::domain_types::AdminNewPassword,
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
}
