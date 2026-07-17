#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

const INSERT_USER: &str =
    "INSERT INTO admin_users (login, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id";
const RECENT_LOGIN_FAILURE_COUNT: &str = "SELECT count(*) FROM admin_login_attempts WHERE login = $1 AND succeeded = false AND attempted_at > now() - interval '15 minutes'";
const SIGN_IN_USER: &str =
    "SELECT id, password_hash, is_banned FROM admin_users WHERE lower(login) = lower($1)";
const LOCK_REFRESH_TOKEN_USER: &str = "SELECT user_id FROM admin_refresh_tokens WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > now() FOR UPDATE";
const REVOKE_REFRESH_TOKEN: &str = "UPDATE admin_refresh_tokens SET revoked_at = now() WHERE token_hash = $1 AND user_id = $2 AND revoked_at IS NULL";
const UPDATE_USER: &str = "UPDATE admin_users SET login = COALESCE($2, login), display_name = COALESCE($3, display_name) WHERE id = $1 RETURNING true";
const UPDATE_USER_PASSWORD: &str =
    "UPDATE admin_users SET password_hash = $2 WHERE id = $1 RETURNING true";
const UPDATE_USER_BAN: &str = "UPDATE admin_users SET is_banned = $2 WHERE id = $1 RETURNING true";
const DELETE_USER: &str = "DELETE FROM admin_users WHERE id = $1 RETURNING true";
const LIST_USERS: &str =
    "SELECT id, login, display_name, is_banned FROM admin_users ORDER BY login LIMIT 500";

pub(crate) async fn insert_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    login: &server_admin_contract::AdminLogin,
    display_name: &server_admin_contract::AdminDisplayName,
    password_hash: &crate::AdminPasswordHash,
) -> Result<crate::AdminUserId, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(INSERT_USER)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.0.as_ref())
        .fetch_one(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(crate::AdminUserId::from)
}

pub(crate) async fn recent_login_failure_count(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::AdminLogin,
) -> Result<super::AdminRecentLoginFailureCount, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(RECENT_LOGIN_FAILURE_COUNT)
        .bind(login.as_ref())
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(super::AdminRecentLoginFailureCount::from)
}

pub(crate) async fn find_sign_in_user(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::AdminLogin,
) -> Result<Option<super::AdminSignInUser>, crate::SqlxAdminError> {
    sqlx::query_as::<_, (i64, String, bool)>(SIGN_IN_USER)
        .bind(login.as_ref())
        .fetch_optional(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| {
            value.map(|(id, password_hash, is_banned)| super::AdminSignInUser {
                id: crate::AdminUserId::from(id),
                password_hash: crate::AdminPasswordHash::new(
                    pg_types_text_misc::StringAsNonNullTextSecret::from(password_hash),
                ),
                is_banned: crate::StdAdminBool::from(is_banned),
            })
        })
}

pub(crate) async fn lock_refresh_token_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::AdminTokenHash,
) -> Result<Option<crate::AdminUserId>, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(LOCK_REFRESH_TOKEN_USER)
        .bind(secrecy::ExposeSecret::expose_secret(token_hash.0.as_ref()))
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| value.map(crate::AdminUserId::from))
}

pub(crate) async fn revoke_refresh_token(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::AdminTokenHash,
    user_id: crate::AdminUserId,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(REVOKE_REFRESH_TOKEN)
        .bind(secrecy::ExposeSecret::expose_secret(token_hash.0.as_ref()))
        .bind(user_id.0)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn update_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    login: Option<&server_admin_contract::AdminLogin>,
    display_name: Option<&server_admin_contract::AdminDisplayName>,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(UPDATE_USER)
        .bind(user_id.0)
        .bind(login.map(|value| value.as_ref().as_str()))
        .bind(display_name.map(|value| value.as_ref().as_str()))
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn update_user_password(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    password_hash: &crate::AdminPasswordHash,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(UPDATE_USER_PASSWORD)
        .bind(user_id.0)
        .bind(password_hash.0.as_ref())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn update_user_ban(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    is_banned: crate::StdAdminBool,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(UPDATE_USER_BAN)
        .bind(user_id.0)
        .bind(is_banned.0)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn delete_user(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(DELETE_USER)
        .bind(user_id.0)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| crate::StdAdminBool::from(value.is_some()))
}

pub(crate) async fn list_users(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<Vec<server_admin_contract::AdminUserSummary>, super::AdminRepositoryError> {
    sqlx::query_as::<_, (i64, String, String, bool)>(LIST_USERS)
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .into_iter()
        .map(|(id, login, display_name, is_banned)| {
            Ok(server_admin_contract::AdminUserSummary::new(
                server_admin_contract::AdminDisplayName::try_from(display_name)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminUserId::from(id),
                server_admin_contract::AdminBool::from(is_banned),
                server_admin_contract::AdminLogin::try_from(login)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect()
}
const READ_AUTH_USER: &str =
    "SELECT login, display_name FROM admin_users WHERE id = $1 AND is_banned = false";
const READ_AUTH_ROLES: &str = "SELECT role.name FROM admin_roles role JOIN admin_user_roles link ON link.role_id = role.id WHERE link.user_id = $1 ORDER BY role.name";
const READ_AUTH_PERMISSIONS: &str = "SELECT DISTINCT permission.name FROM admin_permissions permission JOIN admin_role_permissions role_permission ON role_permission.permission_id = permission.id JOIN admin_user_roles user_role ON user_role.role_id = role_permission.role_id WHERE user_role.user_id = $1 ORDER BY permission.name";

pub(crate) async fn read_authenticated_record(
    db: &mut super::AdminRepositoryDbRef<'_, '_>,
    user_id: crate::AdminUserId,
) -> Result<Option<super::AdminAuthenticatedRecord>, super::AdminRepositoryError> {
    let user_query = sqlx::query_as::<_, (String, String)>(READ_AUTH_USER).bind(user_id.0);
    let optional_user = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            user_query.fetch_optional(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => user_query.fetch_optional(pool.0).await,
    }
    .map_err(crate::SqlxAdminError::from)?;
    let Some((login, display_name)) = optional_user else {
        return Ok(None);
    };
    let roles_query = sqlx::query_scalar::<_, String>(READ_AUTH_ROLES).bind(user_id.0);
    let raw_roles = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            roles_query.fetch_all(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => roles_query.fetch_all(pool.0).await,
    }
    .map_err(crate::SqlxAdminError::from)?;
    let permissions_query = sqlx::query_scalar::<_, String>(READ_AUTH_PERMISSIONS).bind(user_id.0);
    let raw_permissions = match db {
        super::AdminRepositoryDbRef::Connection(connection) => {
            permissions_query.fetch_all(&mut *connection.0).await
        }
        super::AdminRepositoryDbRef::Pool(pool) => permissions_query.fetch_all(pool.0).await,
    }
    .map_err(crate::SqlxAdminError::from)?;
    Ok(Some(super::AdminAuthenticatedRecord {
        display_name: server_admin_contract::AdminDisplayName::try_from(display_name)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        login: server_admin_contract::AdminLogin::try_from(login)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        permissions: raw_permissions
            .into_iter()
            .map(|permission| server_admin_contract::AdminPermission::try_from(permission.as_str()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        roles: raw_roles
            .into_iter()
            .map(server_admin_contract::AdminRoleName::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
    }))
}
