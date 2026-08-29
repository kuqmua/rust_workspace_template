#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn permission_ids_impl(
    value: &crate::admin_html_form_text::AdminHtmlFormText,
) -> Result<
    server_admin_contract::admin_permission_ids::AdminPermissionIds,
    crate::admin_error::AdminError,
> {
    crate::assignment_ids_impl::assignment_ids_impl::<
        server_admin_contract::admin_permission_id::AdminPermissionId,
        _,
        server_admin_contract::admin_permission_ids::AdminPermissionIds,
        _,
    >(value)
}
