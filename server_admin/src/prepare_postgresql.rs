pub async fn prepare_postgresql(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<(), crate::admin_migrate_error::AdminMigrateError> {
    pg_table::ensure_pg_table_idempotency_schema::ensure_pg_table_idempotency_schema(
        sqlx_pg_pool_ref,
    )
    .await
    .map_err(crate::admin_migrate_error::AdminMigrateError::Idempotency)?;
    crate::migrator::migrator()
        .run(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_migrate_error::SqlxAdminMigrateError::from)
        .map_err(crate::admin_migrate_error::AdminMigrateError::from)?;
    let rule_names = server_admin_contract::admin_rule::AdminRule::ALL
        .into_iter()
        .map(|rule| rule.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    let _rule_result = sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_RULES_SQL)
        .bind(rule_names)
        .execute(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_migrate_error::AdminMigrateError::from)?;
    let _role_rule_result = sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_ROLE_RULES_SQL)
        .execute(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_migrate_error::AdminMigrateError::from)?;
    Ok(())
}
