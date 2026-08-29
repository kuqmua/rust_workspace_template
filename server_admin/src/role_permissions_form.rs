use super::{AdminHtmlFormText, StdAdminHtmlSelected};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(crate) struct RolePermissionsForm {
    pub(crate) expected_permission_ids: AdminHtmlFormText,
    #[serde(flatten)]
    pub(crate) selected: StdAdminHtmlSelected,
    pub(crate) role_id: server_admin_contract::domain_types::AdminRoleId,
}
