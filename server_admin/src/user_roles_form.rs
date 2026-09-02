#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    serde::Deserialize,
    proc_macro_getters::Getters,
)]
pub(crate) struct UserRolesForm {
    expected_role_ids: crate::admin_html_form_text::AdminHtmlFormText,
    #[serde(flatten)]
    selected: crate::std_admin_html_selected::StdAdminHtmlSelected,
    user_id: server_admin_contract::admin_user_id::AdminUserId,
}
