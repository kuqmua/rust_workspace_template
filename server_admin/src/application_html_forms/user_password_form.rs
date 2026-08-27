#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::domain_types::auth::html) struct UserPasswordForm {
    pub(in crate::domain_types::auth::html) password:
        server_admin_contract::domain_types::AdminNewPassword,
    pub(in crate::domain_types::auth::html) user_id:
        server_admin_contract::domain_types::AdminUserId,
}
