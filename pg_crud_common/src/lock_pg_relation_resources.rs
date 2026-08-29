pub async fn lock_pg_relation_resources(
    mut connection: crate::sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef<
        '_,
    >,
    namespace: &crate::pg_relation_lock_namespace::PgRelationLockNamespace,
    resources: &crate::pg_relation_resource_ids::PgRelationResourceIds,
) -> Result<(), crate::sqlx_pg_relation_lock_error::SqlxPgRelationLockError> {
    if resources.0.is_empty() {
        return Ok(());
    }
    let resource_values = resources
        .0
        .iter()
        .map(|resource| resource.0)
        .collect::<Vec<_>>();
    let _result = sqlx::query(constants_str::catalog::PG_RELATION_RESOURCE_ADVISORY_LOCK_SQL)
        .bind(namespace.0.as_str())
        .bind(resource_values)
        .execute(connection.as_mut())
        .await
        .map_err(crate::sqlx_pg_relation_lock_error::SqlxPgRelationLockError::from)?;
    Ok(())
}
