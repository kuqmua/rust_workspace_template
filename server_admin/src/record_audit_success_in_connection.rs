pub(crate) async fn record_audit_success_in_connection(
    connection: crate::repository::SqlxAdminRepositoryConnectionMutRef<'_>,
    event: crate::AdminAuditSuccessRef<'_>,
) -> Result<(), crate::AdminError> {
    let details = server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": event.action.as_str().as_ref(), "target_id": event.resource_id.value().as_ref() }),
    )
    .map_err(|_error| crate::AdminError::Validation)?;
    let resource_id = event.resource_id.value();
    crate::repository::insert_audit_success::insert_audit_success(
        connection,
        event.user_id,
        event.login,
        event.action,
        event.resource,
        &resource_id,
        crate::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(crate::AdminError::postgresql)
}
