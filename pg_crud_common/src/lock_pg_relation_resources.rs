pub async fn lock_pg_relation_resources(
    mut connection: crate::domain_types::SqlxPgRelationLockConnectionRef<'_>,
    namespace: &crate::domain_types::PgRelationLockNamespace,
    resources: &crate::domain_types::PgRelationResourceIds,
) -> Result<(), crate::domain_types::SqlxPgRelationLockError> {
    if resources.0.is_empty() {
        return Ok(());
    }
    let resource_values = resources
        .0
        .iter()
        .map(|resource| resource.0)
        .collect::<Vec<_>>();
    let _result = sqlx::query(constants_str::PG_RELATION_RESOURCE_ADVISORY_LOCK_SQL)
        .bind(namespace.0.as_str())
        .bind(resource_values)
        .execute(connection.as_mut())
        .await
        .map_err(crate::domain_types::SqlxPgRelationLockError::from)?;
    Ok(())
}
