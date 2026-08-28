pub(crate) fn role_path_impl(
    value: server_admin_contract::domain_types::AdminRoleId,
) -> crate::AdminRoleId {
    crate::AdminRoleId::from(value.value())
}
