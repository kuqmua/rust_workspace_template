pub(crate) async fn finalize_audited_transaction(
    mut sqlx_admin_transaction: crate::sqlx_admin_transaction::SqlxAdminTransaction<'_>,
    admin_audit_success_ref: crate::admin_audit_success_ref::AdminAuditSuccessRef<'_>,
) -> Result<(), crate::admin_error::AdminError> {
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut **sqlx_admin_transaction,
        ),
        admin_audit_success_ref,
    )
    .await?;
    sqlx::Transaction::from(sqlx_admin_transaction)
        .commit()
        .await
        .map_err(crate::admin_error::AdminError::from)
}
