pub(crate) fn user_path_impl(
    value: server_admin_contract::admin_user_id::AdminUserId,
) -> server_admin_core::admin_user_id::AdminUserId {
    server_admin_core::admin_user_id::AdminUserId::from(value.value())
}
