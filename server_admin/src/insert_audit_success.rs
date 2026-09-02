pub(crate) async fn insert_audit_success(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    admin_login: &server_admin_contract::admin_login::AdminLogin,
    admin_audit_action: crate::admin_audit_action::AdminAuditAction,
    admin_audit_resource: crate::admin_audit_resource::AdminAuditResource,
    std_admin_string: &server_admin_core::std_admin_string::StdAdminString,
    uuid_admin_value: server_admin_core::uuid_admin_value::UuidAdminValue,
    serde_json_admin_audit_details: &server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_AUDIT_SUCCESS_SQL)
        .bind(admin_user_record_id.get())
        .bind(admin_login.as_ref())
        .bind(admin_audit_action.as_str().as_ref())
        .bind(admin_audit_resource.as_str().as_ref())
        .bind(std_admin_string.as_ref())
        .bind(uuid_admin_value.get())
        .bind(serde_json_admin_audit_details.as_ref())
        .execute(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
