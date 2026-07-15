#![allow(clippy::single_call_fn)] // stable root migration/bootstrap API delegates to the private persistence module
static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
#[cfg(test)]
pub(super) const fn migrator() -> &'static sqlx::migrate::Migrator {
    &ADMIN_MIGRATOR
}
pub(super) async fn prep_pg(
    pool: app_state::SqlxPgPoolRef<'_>,
) -> Result<(), super::AdminMigrateError> {
    ADMIN_MIGRATOR
        .run(pool.as_ref())
        .await
        .map_err(|error| super::AdminMigrateError(super::SqlxAdminMigrateError::from(error)))
}
#[allow(clippy::single_call_fn)] // shared validator keeps bootstrap behavior directly unit-testable and aligned with the database constraint
pub(super) fn admin_login_has_valid_format(login: &super::AdminLogin) -> super::StdAdminBool {
    let value: &String = login.as_ref();
    super::StdAdminBool::from(
        value.len() >= 3
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'.' | b'-')
            }),
    )
}
pub(super) async fn bootstrap_admin(
    pool: app_state::SqlxPgPoolRef<'_>,
    login: super::AdminLogin,
    display_name: super::AdminDisplayName,
    password: super::AdminPassword,
    password_hasher: &super::AdminPasswordHasher,
) -> Result<super::AdminUserId, super::AdminBootstrapError> {
    if !admin_login_has_valid_format(&login).0 {
        return Err(super::AdminBootstrapError::InvalidLogin);
    }
    if display_name.as_ref().trim().is_empty() {
        return Err(super::AdminBootstrapError::EmptyDisplayName);
    }
    let password_hash = password_hasher
        .hash(password)
        .await
        .map_err(super::AdminBootstrapError::PasswordHash)?;
    let mut tx = pool
        .as_ref()
        .begin()
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let _lock_result = sqlx::query(str_constants::LOCK_TABLE_ADMIN_USERS_IN_EXCLUSIVE_MODE)
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let user_exists =
        sqlx::query_scalar::<_, bool>(str_constants::SELECT_EXISTS_SELECT_1_FROM_ADMIN_USERS)
            .fetch_one(&mut *tx)
            .await
            .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    if user_exists {
        return Err(super::AdminBootstrapError::AlreadyInitialized);
    }
    let user_id = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.0.as_ref())
        .fetch_one(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let _role_link_result =
        sqlx::query(str_constants::INSERT_INTO_ADMIN_USER_ROLES_USER_ID_ROLE_ID_SELECT_DOLLAR_1)
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    tx.commit()
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    Ok(super::AdminUserId::from(user_id))
}
