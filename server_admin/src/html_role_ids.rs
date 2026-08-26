pub(super) fn role_ids_impl(
    value: &super::forms::AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminRoleIds, super::super::AdminError> {
    super::assignment_ids_impl::assignment_ids_impl::<
        server_admin_contract::domain_types::AdminRoleId,
        _,
        server_admin_contract::domain_types::AdminRoleIds,
        _,
    >(value)
}
