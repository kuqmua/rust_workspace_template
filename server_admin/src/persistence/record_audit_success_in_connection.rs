pub(in super::super) async fn record_audit_success_in_connection(
    mut connection: super::SqlxAdminPgConnectionRef<'_>,
    event: super::AdminAuditSuccessRef<'_>,
) -> Result<(), super::super::AdminError> {
    let details = server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": event.action.as_str().as_ref(), "target_id": event.resource_id.value().as_ref() }),
    )
    .map_err(|_error| super::super::AdminError::Validation)?;
    let resource_id = event.resource_id.value();
    crate::repository::insert_audit_success::insert_audit_success(
        crate::repository::SqlxAdminRepositoryConnectionMutRef::from(connection.as_mut()),
        event.user_id,
        event.login,
        event.action,
        event.resource,
        &resource_id,
        super::super::super::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(super::super::AdminError::postgresql)
}
