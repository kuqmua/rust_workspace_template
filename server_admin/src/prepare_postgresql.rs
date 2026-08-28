use crate::{AdminMigrateError, SqlxAdminMigrateError};

pub async fn prepare_postgresql(
    pool: app_state::SqlxPgPoolRef<'_>,
) -> Result<(), AdminMigrateError> {
    crate::migrations::migrator()
        .run(pool.as_ref())
        .await
        .map_err(SqlxAdminMigrateError::from)
        .map_err(AdminMigrateError::from)?;
    let permission_names = crate::AdminPermission::ALL
        .into_iter()
        .map(|permission| permission.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    let _permission_result = sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_PERMISSIONS_SQL)
        .bind(permission_names)
        .execute(pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminMigrateError::from)?;
    let _role_permission_result =
        sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_ROLE_PERMISSIONS_SQL)
            .execute(pool.as_ref())
            .await
            .map_err(crate::SqlxAdminError::from)
            .map_err(AdminMigrateError::from)?;
    Ok(())
}
