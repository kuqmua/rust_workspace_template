#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(crate) struct RolePermissionsForm {
    pub(crate) expected_permission_ids: crate::admin_html_form_text::AdminHtmlFormText,
    #[serde(flatten)]
    pub(crate) selected: crate::std_admin_html_selected::StdAdminHtmlSelected,
    pub(crate) role_id: server_admin_contract::admin_role_id::AdminRoleId,
}
