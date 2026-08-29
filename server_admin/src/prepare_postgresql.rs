pub async fn prepare_postgresql(
    pool: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<(), crate::admin_migrate_error::AdminMigrateError> {
    crate::migrator::migrator()
        .run(pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_migrate_error::SqlxAdminMigrateError::from)
        .map_err(crate::admin_migrate_error::AdminMigrateError::from)?;
    let permission_names = server_admin_contract::admin_permission::AdminPermission::ALL
        .into_iter()
        .map(|permission| permission.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    let _permission_result =
        sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_RECONCILE_PERMISSIONS_SQL)
            .bind(permission_names)
            .execute(pool.as_ref())
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map_err(crate::admin_migrate_error::AdminMigrateError::from)?;
    let _role_permission_result = sqlx::query(
        constants_str::integration_fixtures::SERVER_ADMIN_RECONCILE_ROLE_PERMISSIONS_SQL,
    )
    .execute(pool.as_ref())
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_migrate_error::AdminMigrateError::from)?;
    Ok(())
}
