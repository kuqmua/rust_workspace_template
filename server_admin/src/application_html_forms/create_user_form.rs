#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::domain_types::auth::html) struct CreateUserForm {
    pub(in crate::domain_types::auth::html) display_name:
        server_admin_contract::domain_types::AdminDisplayName,
    pub(in crate::domain_types::auth::html) login: server_admin_contract::domain_types::AdminLogin,
    pub(in crate::domain_types::auth::html) password:
        server_admin_contract::domain_types::AdminNewPassword,
}
