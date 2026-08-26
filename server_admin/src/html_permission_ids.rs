pub(super) fn permission_ids(
    value: &super::forms::AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminPermissionIds, super::super::AdminError> {
    super::assignment_ids_impl::assignment_ids::<
        server_admin_contract::domain_types::AdminPermissionId,
        _,
        server_admin_contract::domain_types::AdminPermissionIds,
        _,
    >(value)
}
