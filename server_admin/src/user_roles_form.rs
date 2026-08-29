use super::{AdminHtmlFormText, StdAdminHtmlSelected};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(crate) struct UserRolesForm {
    pub(crate) expected_role_ids: AdminHtmlFormText,
    #[serde(flatten)]
    pub(crate) selected: StdAdminHtmlSelected,
    pub(crate) user_id: server_admin_contract::domain_types::AdminUserId,
}
