use super::{AdminHtmlFormText, StdAdminHtmlSelected};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(super) struct UserRolesForm {
    pub(super) expected_role_ids: AdminHtmlFormText,
    #[serde(flatten)]
    pub(super) selected: StdAdminHtmlSelected,
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
}
