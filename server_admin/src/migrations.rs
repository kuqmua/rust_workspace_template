#![allow(clippy::single_call_fn)] // stable root migration/bootstrap API delegates to the private persistence module
static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
const RECONCILE_PERMISSIONS: &str =
    "insert into admin_permissions (name) select unnest($1::text[]) on conflict (name) do nothing";
const RECONCILE_ROLE_PERMISSIONS: &str = "insert into admin_role_permissions (role_id, permission_id) select admin_roles.id, admin_permissions.id from admin_roles cross join admin_permissions where admin_roles.name = 'admin' on conflict (role_id, permission_id) do nothing";
const LOCK_USERS: &str = "LOCK TABLE admin_users IN EXCLUSIVE MODE";
const USERS_EXIST: &str = "SELECT EXISTS (SELECT 1 FROM admin_users)";
const INSERT_ADMIN_ROLE: &str = "INSERT INTO admin_user_roles (user_id, role_id) SELECT $1, id FROM admin_roles WHERE name = 'admin'";
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
    let _permission_result = sqlx::query(RECONCILE_PERMISSIONS)
        .bind(permission_names)
        .execute(pool.as_ref())
        .await
        .map_err(|error| {
            super::AdminMigrateError(super::AdminMigrateErrorInner::Reconciliation(
                super::SqlxAdminError::from(error),
            ))
        })?;
    let _role_permission_result = sqlx::query(RECONCILE_ROLE_PERMISSIONS)
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
    let _lock_result = sqlx::query(LOCK_USERS)
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let user_exists = sqlx::query_scalar::<_, bool>(USERS_EXIST)
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
    let _role_link_result = sqlx::query(INSERT_ADMIN_ROLE)
        .bind(user_id.0)
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    tx.commit()
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    Ok(user_id)
}
