pub(crate) async fn insert_audit_success(
    connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: server_admin_core::admin_user_id::AdminUserId,
    login: &server_admin_contract::admin_login::AdminLogin,
    action: crate::admin_audit_action::AdminAuditAction,
    resource: crate::admin_audit_resource::AdminAuditResource,
    resource_id: &server_admin_core::std_admin_string::StdAdminString,
    request_id: server_admin_core::uuid_admin_value::UuidAdminValue,
    details: &server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_INSERT_AUDIT_SUCCESS_SQL)
        .bind(user_id.get())
        .bind(login.as_ref())
        .bind(action.as_str().as_ref())
        .bind(resource.as_str().as_ref())
        .bind(resource_id.as_ref())
        .bind(request_id.get())
        .bind(details.as_ref())
        .execute(connection.0)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
