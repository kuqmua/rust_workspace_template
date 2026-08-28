#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn permission_ids_impl(
    value: &crate::AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminPermissionIds, crate::AdminError> {
    crate::assignment_ids_impl::assignment_ids_impl::<
        server_admin_contract::domain_types::AdminPermissionId,
        _,
        server_admin_contract::domain_types::AdminPermissionIds,
        _,
    >(value)
}
