pub(super) fn role_path_impl(
    value: server_admin_contract::domain_types::AdminRoleId,
) -> super::super::super::AdminRoleId {
    super::super::super::AdminRoleId::from(value.value())
}
