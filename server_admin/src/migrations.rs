#![allow(clippy::single_call_fn)] // stable root migration/bootstrap API delegates to the private persistence module
static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
#[cfg(test)]
pub(super) const fn migrator() -> &'static sqlx::migrate::Migrator {
    &ADMIN_MIGRATOR
}
pub(super) async fn prep_pg(
    pool: app_state::SqlxPgPoolRef<'_>,
) -> Result<(), super::AdminMigrateEr> {
    ADMIN_MIGRATOR
        .run(pool.as_ref())
        .await
        .map_err(|er| super::AdminMigrateEr(super::SqlxAdminMigrateEr::from(er)))
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
) -> Result<super::AdminUserId, super::AdminBootstrapEr> {
    if !admin_login_has_valid_format(&login).0 {
        return Err(super::AdminBootstrapEr::InvalidLogin);
    }
    if display_name.as_ref().trim().is_empty() {
        return Err(super::AdminBootstrapEr::EmptyDisplayName);
    }
    let password_hash = password_hasher
        .hash(password)
        .await
        .map_err(super::AdminBootstrapEr::PasswordHash)?;
    let mut tx = pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| super::AdminBootstrapEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _lock_result = sqlx::query("LOCK TABLE admin_users IN EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(|er| super::AdminBootstrapEr::Pg(super::SqlxAdminEr::from(er)))?;
    let user_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM admin_users)")
        .fetch_one(&mut *tx)
        .await
        .map_err(|er| super::AdminBootstrapEr::Pg(super::SqlxAdminEr::from(er)))?;
    if user_exists {
        return Err(super::AdminBootstrapEr::AlreadyInitialized);
    }
    let user_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO admin_users (login, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(login.as_ref())
    .bind(display_name.as_ref())
    .bind(password_hash.0.as_ref())
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| super::AdminBootstrapEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _role_link_result = sqlx::query(
        "INSERT INTO admin_user_roles (user_id, role_id) SELECT $1, id FROM admin_roles WHERE name = 'admin'",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|er| super::AdminBootstrapEr::Pg(super::SqlxAdminEr::from(er)))?;
    tx.commit()
        .await
        .map_err(|er| super::AdminBootstrapEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(super::AdminUserId::from(user_id))
}
