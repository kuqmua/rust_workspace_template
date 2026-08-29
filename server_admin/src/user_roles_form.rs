#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(crate) struct UserRolesForm {
    pub(crate) expected_role_ids: crate::admin_html_form_text::AdminHtmlFormText,
    #[serde(flatten)]
    pub(crate) selected: crate::std_admin_html_selected::StdAdminHtmlSelected,
    pub(crate) user_id: server_admin_contract::admin_user_id::AdminUserId,
}
