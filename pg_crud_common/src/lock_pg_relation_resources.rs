pub async fn lock_pg_relation_resources(
    mut connection: crate::sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef<
        '_,
    >,
    namespace: &crate::pg_relation_lock_namespace::PgRelationLockNamespace,
    resources: &crate::pg_relation_resource_ids::PgRelationResourceIds,
) -> Result<(), crate::sqlx_pg_relation_lock_error::SqlxPgRelationLockError> {
    if resources.get_inner().is_empty() {
        return Ok(());
    }
    let resource_values = resources
        .get_inner()
        .iter()
        .map(|resource| *resource.get_inner())
        .collect::<Vec<_>>();
    let _result = sqlx::query(constants_str::PG_RELATION_RESOURCE_ADVISORY_LOCK_SQL)
        .bind(namespace.get_inner().as_str())
        .bind(resource_values)
        .execute(connection.as_mut())
        .await
        .map_err(crate::sqlx_pg_relation_lock_error::SqlxPgRelationLockError::from)?;
    Ok(())
}
