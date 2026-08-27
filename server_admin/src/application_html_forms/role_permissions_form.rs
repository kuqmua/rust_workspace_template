use super::{AdminHtmlFormText, StdAdminHtmlSelected};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(in crate::domain_types::auth::html) struct RolePermissionsForm {
    pub(in crate::domain_types::auth::html) expected_permission_ids: AdminHtmlFormText,
    #[serde(flatten)]
    pub(in crate::domain_types::auth::html) selected: StdAdminHtmlSelected,
    pub(in crate::domain_types::auth::html) role_id:
        server_admin_contract::domain_types::AdminRoleId,
}
