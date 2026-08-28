use super::{AdminHtmlFormText, StdAdminHtmlSelected};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(in crate::domain_types::auth::html) struct UserRolesForm {
    pub(in crate::domain_types::auth::html) expected_role_ids: AdminHtmlFormText,
    #[serde(flatten)]
    pub(in crate::domain_types::auth::html) selected: StdAdminHtmlSelected,
    pub(in crate::domain_types::auth::html) user_id:
        server_admin_contract::domain_types::AdminUserId,
}
