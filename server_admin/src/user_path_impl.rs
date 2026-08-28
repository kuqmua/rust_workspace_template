pub(crate) fn user_path_impl(
    value: server_admin_contract::domain_types::AdminUserId,
) -> crate::AdminUserId {
    crate::AdminUserId::from(value.value())
}
