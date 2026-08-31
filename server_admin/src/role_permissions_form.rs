#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    serde::Deserialize,
    generate_accessor::Getters,
)]
pub(crate) struct RolePermissionsForm {
    expected_permission_ids: crate::admin_html_form_text::AdminHtmlFormText,
    #[serde(flatten)]
    selected: crate::std_admin_html_selected::StdAdminHtmlSelected,
    role_id: server_admin_contract::admin_role_id::AdminRoleId,
}
