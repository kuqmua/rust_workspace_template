#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::domain_types::auth::html) struct UpdateRoleForm {
    pub(in crate::domain_types::auth::html) name:
        server_admin_contract::domain_types::AdminRoleName,
    pub(in crate::domain_types::auth::html) role_id:
        server_admin_contract::domain_types::AdminRoleId,
}
