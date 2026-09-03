#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn permission_ids_impl(
    admin_html_form_text: &crate::admin_html_form_text::AdminHtmlFormText,
) -> Result<
    server_admin_contract::admin_permission_ids::AdminPermissionIds,
    crate::admin_error::AdminError,
> {
    crate::assignment_ids_impl::assignment_ids_impl::<
        server_admin_contract::admin_permission_id::AdminPermissionId,
        _,
        server_admin_contract::admin_permission_ids::AdminPermissionIds,
        _,
    >(admin_html_form_text)
}
