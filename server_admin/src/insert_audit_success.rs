pub(crate) async fn insert_audit_success(
    connection: crate::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    login: &server_admin_contract::domain_types::AdminLogin,
    action: crate::domain_types::AdminAuditAction,
    resource: crate::domain_types::AdminAuditResource,
    resource_id: &crate::domain_types::StdAdminString,
    request_id: crate::domain_types::UuidAdminValue,
    details: &server_admin_contract::domain_types::SerdeJsonAdminAuditDetails,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_AUDIT_SUCCESS_SQL)
        .bind(user_id.get())
        .bind(login.as_ref())
        .bind(action.as_str().as_ref())
        .bind(resource.as_str().as_ref())
        .bind(resource_id.as_ref())
        .bind(request_id.get())
        .bind(details.as_ref())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}
