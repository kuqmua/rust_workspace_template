pub(crate) async fn record_audit_success_in_connection(
    connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    event: crate::admin_audit_success_ref::AdminAuditSuccessRef<'_>,
) -> Result<(), crate::admin_error::AdminError> {
    let details = server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": event.get_action().as_str().as_ref(), "target_id": event.get_resource_id().value().as_ref() }),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    let resource_id = event.get_resource_id().value();
    crate::insert_audit_success::insert_audit_success(
        connection,
        *event.get_user_id(),
        event.get_login(),
        *event.get_action(),
        *event.get_resource(),
        &resource_id,
        server_admin_core::uuid_admin_value::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(crate::admin_error::AdminError::postgresql)
}
