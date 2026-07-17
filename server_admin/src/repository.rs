const INSERT_USER: &str =
    "INSERT INTO admin_users (login, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id";
const REVOKE_ACCESS_SESSION: &str = "UPDATE admin_access_sessions SET revoked_at = now() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL";
const REVOKE_USER_ACCESS_SESSIONS: &str =
    "UPDATE admin_access_sessions SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL";
const REVOKE_USER_REFRESH_TOKENS: &str =
    "UPDATE admin_refresh_tokens SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL";
const LOCK_LAST_ADMIN: &str =
    "SELECT pg_advisory_xact_lock(hashtext('admin_last_active_administrator'))";
const USER_IS_ADMIN: &str = "SELECT EXISTS (SELECT 1 FROM admin_user_roles user_role JOIN admin_roles role ON role.id = user_role.role_id WHERE user_role.user_id = $1 AND role.name = 'admin')";
const ACTIVE_ADMIN_COUNT: &str = "SELECT count(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id JOIN admin_roles role ON role.id = user_role.role_id WHERE role.name = 'admin' AND users.is_banned = false";

pub(crate) async fn insert_user(
    connection: &mut sqlx::PgConnection,
    login: &server_admin_contract::AdminLogin,
    display_name: &server_admin_contract::AdminDisplayName,
    password_hash: &super::AdminPasswordHash,
) -> Result<super::AdminUserId, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(INSERT_USER)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.0.as_ref())
        .fetch_one(connection)
        .await
        .map(super::AdminUserId::from)
}

pub(crate) async fn revoke_access_session(
    connection: &mut sqlx::PgConnection,
    session_id: super::AdminSessionId,
    user_id: super::AdminUserId,
) -> Result<(), sqlx::Error> {
    sqlx::query(REVOKE_ACCESS_SESSION)
        .bind(session_id.0.0)
        .bind(user_id.0)
        .execute(connection)
        .await
        .map(drop)
}

pub(crate) async fn revoke_user_sessions(
    connection: &mut sqlx::PgConnection,
    user_id: super::AdminUserId,
) -> Result<(), sqlx::Error> {
    sqlx::query(REVOKE_USER_ACCESS_SESSIONS)
        .bind(user_id.0)
        .execute(&mut *connection)
        .await
        .map(drop)?;
    sqlx::query(REVOKE_USER_REFRESH_TOKENS)
        .bind(user_id.0)
        .execute(connection)
        .await
        .map(drop)
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LastAdminState {
    pub(crate) active_count: i64,
    pub(crate) target_is_admin: bool,
}

pub(crate) async fn lock_and_read_last_admin_state(
    connection: &mut sqlx::PgConnection,
    user_id: super::AdminUserId,
) -> Result<LastAdminState, sqlx::Error> {
    lock_last_admin(connection).await?;
    read_last_admin_state(connection, user_id).await
}

pub(crate) async fn lock_last_admin(
    connection: &mut sqlx::PgConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(LOCK_LAST_ADMIN)
        .execute(connection)
        .await
        .map(drop)
}

pub(crate) async fn read_last_admin_state(
    connection: &mut sqlx::PgConnection,
    user_id: super::AdminUserId,
) -> Result<LastAdminState, sqlx::Error> {
    let target_is_admin = sqlx::query_scalar::<_, bool>(USER_IS_ADMIN)
        .bind(user_id.0)
        .fetch_one(&mut *connection)
        .await?;
    let active_count = sqlx::query_scalar::<_, i64>(ACTIVE_ADMIN_COUNT)
        .fetch_one(connection)
        .await?;
    Ok(LastAdminState {
        active_count,
        target_is_admin,
    })
}
