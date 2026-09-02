pub(crate) async fn record_audit_success_in_connection(
    sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_audit_success_ref: crate::admin_audit_success_ref::AdminAuditSuccessRef<'_>,
) -> Result<(), crate::admin_error::AdminError> {
    let details = server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": admin_audit_success_ref.get_action().as_str().as_ref(), "target_id": admin_audit_success_ref.get_resource_id().value().as_ref() }),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let resource_id = admin_audit_success_ref.get_resource_id().value();
    crate::insert_audit_success::insert_audit_success(
        sqlx_admin_repository_connection_mut_ref,
        *admin_audit_success_ref.get_user_id(),
        admin_audit_success_ref.get_login(),
        *admin_audit_success_ref.get_action(),
        *admin_audit_success_ref.get_resource(),
        &resource_id,
        server_admin_core::uuid_admin_value::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(crate::admin_error::AdminError::postgresql)
}
