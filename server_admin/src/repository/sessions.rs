#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn read_active_user_login(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<Option<server_admin_contract::AdminLogin>, super::AdminRepositoryError> {
    sqlx::query_scalar::<_, String>(str_constants::SERVER_ADMIN_READ_ACTIVE_USER_LOGIN_SQL)
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
    current_session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
    query: &server_admin_contract::AdminTableQuery,
) -> Result<server_admin_contract::AdminSessionsPage, super::AdminRepositoryError> {
    let total = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_COUNT_ACTIVE_SESSIONS_SQL)
        .bind(user_id.0)
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let items = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        str_constants::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
    )
    .bind(user_id.0)
    .bind(i64::from(u16::from(query.limit())))
    .bind(i64::from(u32::from(query.offset())))
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
            server_admin_contract::AdminBool::from(id == current_session_id.0.0),
        ))
    })
    .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok(server_admin_contract::AdminSessionsPage::new(
        items.into(),
        super::page_total(super::AdminPageTotalCount::from(total))?,
    ))
}

pub(crate) async fn access_session_is_active(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    session_id: crate::AdminSessionId,
    user_id: crate::AdminUserId,
    context_hash: &crate::AdminTokenHash,
) -> Result<crate::StdAdminBool, crate::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(str_constants::SERVER_ADMIN_ACTIVE_ACCESS_SESSION_SQL)
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
    sqlx::query_scalar::<_, String>(str_constants::SERVER_ADMIN_READ_CSRF_HASH_SQL)
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
    let _access_result = sqlx::query(str_constants::SERVER_ADMIN_REVOKE_EXCESS_ACCESS_SESSIONS_SQL)
        .bind(user_id.0)
        .bind(session_offset)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    if revoke_refresh.0 {
        let _refresh_result =
            sqlx::query(str_constants::SERVER_ADMIN_REVOKE_EXCESS_REFRESH_TOKENS_SQL)
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
    sqlx::query(str_constants::SERVER_ADMIN_INSERT_ACCESS_SESSION_SQL)
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
    sqlx::query(str_constants::SERVER_ADMIN_INSERT_REFRESH_TOKEN_SQL)
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
    sqlx::query(str_constants::SERVER_ADMIN_REVOKE_ACCESS_SESSION_SQL)
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
    sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL)
        .bind(user_id.0)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)?;
    sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(user_id.0)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}
pub(crate) async fn revoke_other_access_sessions(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    current_session_id: crate::AdminSessionId,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(str_constants::SERVER_ADMIN_REVOKE_OTHER_ACCESS_SESSIONS_SQL)
        .bind(user_id.0)
        .bind(current_session_id.0.0)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}
pub(crate) async fn revoke_user_refresh_tokens(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(str_constants::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(user_id.0)
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}
