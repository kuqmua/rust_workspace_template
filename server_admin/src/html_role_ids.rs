pub(super) fn role_ids(
    value: &super::forms::AdminHtmlFormText,
) -> Result<server_admin_contract::domain_types::AdminRoleIds, super::super::AdminError> {
    super::assignment_ids_impl::assignment_ids::<
        server_admin_contract::domain_types::AdminRoleId,
        _,
        server_admin_contract::domain_types::AdminRoleIds,
        _,
    >(value)
}
