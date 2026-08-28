pub(crate) fn role_ids_impl(
    value: &crate::AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminRoleIds, crate::AdminError> {
    crate::assignment_ids_impl::assignment_ids_impl::<
        server_admin_contract::domain_types::AdminRoleId,
        _,
        server_admin_contract::domain_types::AdminRoleIds,
        _,
    >(value)
}
