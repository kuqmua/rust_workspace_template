#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::domain_types::auth::html) struct ChangePasswordForm {
    pub(in crate::domain_types::auth::html) current_password:
        server_admin_contract::domain_types::AdminPassword,
    pub(in crate::domain_types::auth::html) new_password:
        server_admin_contract::domain_types::AdminNewPassword,
}
