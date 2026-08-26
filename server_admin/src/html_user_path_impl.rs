pub(super) fn user_path_impl(
    value: server_admin_contract::domain_types::AdminUserId,
) -> super::super::super::AdminUserId {
    super::super::super::AdminUserId::from(value.value())
}
