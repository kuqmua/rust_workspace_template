#![allow(clippy::single_call_fn)] // stable root migration/bootstrap API delegates to the private persistence module
static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
#[cfg(test)]
pub(super) const fn migrator() -> &'static sqlx::migrate::Migrator {
    &ADMIN_MIGRATOR
}
pub(super) async fn prep_pg(
    pool: app_state::SqlxPgPoolRef<'_>,
) -> Result<(), super::AdminMigrateError> {
    ADMIN_MIGRATOR.run(pool.as_ref()).await.map_err(|error| {
        super::AdminMigrateError(super::AdminMigrateErrorInner::Migration(
            super::SqlxAdminMigrateError::from(error),
        ))
    })?;
    let permission_names = server_admin_contract::AdminPermission::ALL
        .into_iter()
        .map(|permission| permission.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    let _permission_result = sqlx::query(str_constants::SERVER_ADMIN_RECONCILE_PERMISSIONS_SQL)
        .bind(permission_names)
        .execute(pool.as_ref())
        .await
        .map_err(|error| {
            super::AdminMigrateError(super::AdminMigrateErrorInner::Reconciliation(
                super::SqlxAdminError::from(error),
            ))
        })?;
    let _role_permission_result =
        sqlx::query(str_constants::SERVER_ADMIN_RECONCILE_ROLE_PERMISSIONS_SQL)
            .execute(pool.as_ref())
            .await
            .map_err(|error| {
                super::AdminMigrateError(super::AdminMigrateErrorInner::Reconciliation(
                    super::SqlxAdminError::from(error),
                ))
            })?;
    Ok(())
}
pub(super) async fn bootstrap_admin(
    pool: app_state::SqlxPgPoolRef<'_>,
    login: super::AdminLogin,
    display_name: super::AdminDisplayName,
    password: super::AdminPassword,
    password_hasher: &super::AdminPasswordHasher,
) -> Result<super::AdminUserId, super::AdminBootstrapError> {
    let password_hash = password_hasher
        .hash(password)
        .await
        .map_err(super::AdminBootstrapError::PasswordHash)?;
    let mut tx = pool
        .as_ref()
        .begin()
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let _lock_result = sqlx::query(str_constants::SERVER_ADMIN_LOCK_USERS_SQL)
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let user_exists = sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_USERS_EXIST_SQL)
        .fetch_one(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    if user_exists {
        return Err(super::AdminBootstrapError::AlreadyInitialized);
    }
    let user_id = super::repository::users::insert_user(
        super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(super::AdminBootstrapError::Pg)?;
    let _role_link_result = sqlx::query(str_constants::SERVER_ADMIN_INSERT_ADMIN_ROLE_SQL)
        .bind(user_id.0)
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    tx.commit()
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    Ok(user_id)
}
