pub(crate) fn role_path_impl(
    value: server_admin_contract::admin_role_id::AdminRoleId,
) -> server_admin_core::admin_role_record_id::AdminRoleRecordId {
    server_admin_core::admin_role_record_id::AdminRoleRecordId::from(value.value())
}
