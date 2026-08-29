#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn role_ids_impl(
    value: &crate::admin_html_form_text::AdminHtmlFormText,
) -> Result<server_admin_contract::admin_role_ids::AdminRoleIds, crate::admin_error::AdminError> {
    crate::assignment_ids_impl::assignment_ids_impl::<
        server_admin_contract::admin_role_id::AdminRoleId,
        _,
        server_admin_contract::admin_role_ids::AdminRoleIds,
        _,
    >(value)
}
