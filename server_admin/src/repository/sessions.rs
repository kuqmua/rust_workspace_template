#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

const REVOKE_ACCESS_SESSION: &str = "UPDATE admin_access_sessions SET revoked_at = now() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL";
const REVOKE_USER_ACCESS_SESSIONS: &str =
    "UPDATE admin_access_sessions SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL";
const REVOKE_USER_REFRESH_TOKENS: &str =
    "UPDATE admin_refresh_tokens SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL";
const READ_ACTIVE_USER_LOGIN: &str =
    "SELECT login FROM admin_users WHERE id = $1 AND is_banned = false";
const LIST_ACTIVE_SESSIONS: &str = "SELECT id, created_at::text, expires_at::text FROM admin_access_sessions WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > now() ORDER BY created_at DESC";
const ACTIVE_ACCESS_SESSION: &str = "SELECT EXISTS (SELECT 1 FROM admin_access_sessions session JOIN admin_users users ON users.id = session.user_id WHERE session.id = $1 AND session.user_id = $2 AND session.token_context_hash = $3 AND session.revoked_at IS NULL AND session.expires_at > now() AND users.is_banned = false)";
const READ_CSRF_HASH: &str = "SELECT csrf_token_hash FROM admin_access_sessions WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL AND expires_at > now()";
const REVOKE_EXCESS_ACCESS_SESSIONS: &str = "UPDATE admin_access_sessions SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_access_sessions WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)";
const REVOKE_EXCESS_REFRESH_TOKENS: &str = "UPDATE admin_refresh_tokens SET revoked_at = now() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)";
const INSERT_ACCESS_SESSION: &str = "INSERT INTO admin_access_sessions (id, user_id, token_identifier_hash, token_context_hash, csrf_token_hash, expires_at) VALUES ($1, $2, $3, $4, $5, now() + ($6 * interval '1 second'))";
const INSERT_REFRESH_TOKEN: &str = "INSERT INTO admin_refresh_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, now() + ($4 * interval '1 second'))";

pub(crate) async fn read_active_user_login(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<Option<server_admin_contract::AdminLogin>, super::AdminRepositoryError> {
    sqlx::query_scalar::<_, String>(READ_ACTIVE_USER_LOGIN)
        .bind(user_id.0)
        .fetch_optional(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .map(server_admin_contract::AdminLogin::try_from)
        .transpose()
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
}

pub(crate) async fn list_active_sessions(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<Vec<server_admin_contract::AdminSessionView>, super::AdminRepositoryError> {
    sqlx::query_as::<_, (uuid::Uuid, String, String)>(LIST_ACTIVE_SESSIONS)
        .bind(user_id.0)
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .into_iter()
        .map(|(id, created_at, expires_at)| {
            Ok(server_admin_contract::AdminSessionView::new(
                server_admin_contract::AdminSessionTimestamp::try_from(created_at)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminSessionTimestamp::try_from(expires_at)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::AdminSessionIdentifier::try_from(id.to_string())
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect()
}

pub(crate) async fn access_session_is_active(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
    context_hash: &crate::AdminTokenHash,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(ACTIVE_ACCESS_SESSION)
        .bind(session_id.0.0)
        .bind(user_id.0)
        .bind(secrecy::ExposeSecret::expose_secret(
            context_hash.0.as_ref(),
        ))
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(crate::StdAdminBool::from)
}

pub(crate) async fn read_csrf_hash(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
) -> Result<Option<crate::AdminTokenHash>, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, String>(READ_CSRF_HASH)
        .bind(session_id.0.0)
        .bind(user_id.0)
        .fetch_optional(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(|value| {
            value.map(|hash| {
                crate::AdminTokenHash::new(crate::SecrecyAdminString::from(
                    secrecy::SecretBox::new(Box::new(hash)),
                ))
            })
        })
}

pub(crate) async fn enforce_session_limit(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    session_limit: crate::auth::StdAdminSessionLimit,
    revoke_refresh: crate::StdAdminBool,
) -> Result<(), crate::SqlxAdminError> {
    let session_offset =
        i64::try_from(usize::from(session_limit).saturating_sub(1usize)).unwrap_or(i64::MAX);
    let _access_result = sqlx::query(REVOKE_EXCESS_ACCESS_SESSIONS)
        .bind(user_id.0)
        .bind(session_offset)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    if revoke_refresh.0 {
        let _refresh_result = sqlx::query(REVOKE_EXCESS_REFRESH_TOKENS)
            .bind(user_id.0)
            .bind(session_offset)
            .execute(connection.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)] // the access-session row has six independently typed persisted fields
pub(crate) async fn insert_access_session(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
    token_identifier_hash: &crate::AdminTokenHash,
    context_hash: &crate::AdminTokenHash,
    csrf_hash: &crate::AdminTokenHash,
    access_ttl: crate::auth::StdAdminAccessTtlSeconds,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(INSERT_ACCESS_SESSION)
        .bind(session_id.0.0)
        .bind(user_id.0)
        .bind(token_identifier_hash.expose().as_ref())
        .bind(context_hash.expose().as_ref())
        .bind(csrf_hash.expose().as_ref())
        .bind(i64::try_from(u64::from(access_ttl)).unwrap_or(i64::MAX))
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn insert_refresh_token(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    refresh_id: crate::UuidAdminValue,
    user_id: crate::AdminUserId,
    refresh_hash: &crate::AdminTokenHash,
    refresh_ttl: crate::auth::StdAdminRefreshTtlSeconds,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(INSERT_REFRESH_TOKEN)
        .bind(refresh_id.0)
        .bind(user_id.0)
        .bind(refresh_hash.expose().as_ref())
        .bind(i64::try_from(u64::from(refresh_ttl)).unwrap_or(i64::MAX))
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn revoke_access_session(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(REVOKE_ACCESS_SESSION)
        .bind(session_id.0.0)
        .bind(user_id.0)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn revoke_user_sessions(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(REVOKE_USER_ACCESS_SESSIONS)
        .bind(user_id.0)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)?;
    sqlx::query(REVOKE_USER_REFRESH_TOKENS)
        .bind(user_id.0)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}
