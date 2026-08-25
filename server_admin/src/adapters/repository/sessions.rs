#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn read_active_user_login(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<Option<server_admin_contract::domain_types::AdminLogin>, super::AdminRepositoryError> {
    sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_ACTIVE_USER_LOGIN_SQL)
        .bind(user_id.get())
        .fetch_optional(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .map(server_admin_contract::domain_types::AdminLogin::try_from)
        .transpose()
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
}

pub(crate) async fn list_active_sessions(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    current_session_id: crate::domain_types::AdminSessionId,
    user_id: crate::domain_types::AdminUserId,
    query: &server_admin_contract::domain_types::AdminTableQuery,
) -> Result<server_admin_contract::domain_types::AdminSessionsPage, super::AdminRepositoryError> {
    let total = sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_ACTIVE_SESSIONS_SQL)
        .bind(user_id.get())
        .fetch_one(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let items = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        constants_str::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
    )
    .bind(user_id.get())
    .bind(i64::from(u16::from(query.limit())))
    .bind(i64::from(u32::from(query.offset())))
    .fetch_all(pool.0)
    .await
    .map_err(crate::domain_types::SqlxAdminError::from)?
    .into_iter()
    .map(|(id, created_at, expires_at)| {
        Ok(server_admin_contract::domain_types::AdminSessionView::new(
            server_admin_contract::domain_types::AdminSessionTimestamp::try_from(created_at)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            server_admin_contract::domain_types::AdminSessionTimestamp::try_from(expires_at)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            server_admin_contract::domain_types::AdminSessionIdentifier::try_from(id.to_string())
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            server_admin_contract::domain_types::AdminBool::from(
                id == current_session_id.get().get(),
            ),
        ))
    })
    .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok(server_admin_contract::domain_types::AdminSessionsPage::new(
        server_admin_contract::domain_types::AdminSessionViews::try_from(items)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        super::page_total(super::AdminPageTotalCount::from(total))?,
    ))
}

pub(crate) async fn access_session_is_active(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    session_id: crate::domain_types::AdminSessionId,
    user_id: crate::domain_types::AdminUserId,
    context_hash: &crate::domain_types::AdminTokenHash,
) -> Result<crate::domain_types::StdAdminBool, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_ACTIVE_ACCESS_SESSION_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .bind(context_hash.expose().as_ref())
        .fetch_one(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(crate::domain_types::StdAdminBool::from)
}

pub(crate) async fn read_csrf_hash(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    session_id: crate::domain_types::AdminSessionId,
    user_id: crate::domain_types::AdminUserId,
) -> Result<Option<crate::domain_types::AdminTokenHash>, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_CSRF_HASH_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .fetch_optional(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            value
                .map(|hash| {
                    crate::domain_types::SecrecyAdminString::try_from(hash)
                        .map(crate::domain_types::AdminTokenHash::new)
                        .map_err(|error| {
                            crate::domain_types::SqlxAdminError::from(sqlx::Error::Protocol(
                                error.to_string(),
                            ))
                        })
                })
                .transpose()
        })
}

pub(crate) async fn enforce_session_limit(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    session_limit: crate::domain_types::auth::StdAdminSessionLimit,
    revoke_refresh: crate::domain_types::StdAdminBool,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    let session_offset =
        i64::try_from(usize::from(session_limit).saturating_sub(constants_usize::ONE))
            .unwrap_or(i64::MAX);
    let _access_result = sqlx::query(constants_str::SERVER_ADMIN_REVOKE_EXCESS_ACCESS_SESSIONS_SQL)
        .bind(user_id.get())
        .bind(session_offset)
        .execute(&mut *connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    if revoke_refresh.get() {
        let _refresh_result =
            sqlx::query(constants_str::SERVER_ADMIN_REVOKE_EXCESS_REFRESH_TOKENS_SQL)
                .bind(user_id.get())
                .bind(session_offset)
                .execute(connection.0)
                .await
                .map_err(crate::domain_types::SqlxAdminError::from)?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)] // the access-session row has six independently typed persisted fields
pub(crate) async fn insert_access_session(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    session_id: crate::domain_types::AdminSessionId,
    user_id: crate::domain_types::AdminUserId,
    token_identifier_hash: &crate::domain_types::AdminTokenHash,
    context_hash: &crate::domain_types::AdminTokenHash,
    csrf_hash: &crate::domain_types::AdminTokenHash,
    access_ttl: crate::domain_types::auth::StdAdminAccessTtlSeconds,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_ACCESS_SESSION_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .bind(token_identifier_hash.expose().as_ref())
        .bind(context_hash.expose().as_ref())
        .bind(csrf_hash.expose().as_ref())
        .bind(i64::try_from(u64::from(access_ttl)).unwrap_or(i64::MAX))
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn insert_refresh_token(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    refresh_id: crate::domain_types::UuidAdminValue,
    user_id: crate::domain_types::AdminUserId,
    refresh_hash: &crate::domain_types::AdminTokenHash,
    refresh_ttl: crate::domain_types::auth::StdAdminRefreshTtlSeconds,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_REFRESH_TOKEN_SQL)
        .bind(refresh_id.get())
        .bind(user_id.get())
        .bind(refresh_hash.expose().as_ref())
        .bind(i64::try_from(u64::from(refresh_ttl)).unwrap_or(i64::MAX))
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn revoke_access_session(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    session_id: crate::domain_types::AdminSessionId,
    user_id: crate::domain_types::AdminUserId,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_ACCESS_SESSION_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn revoke_user_sessions(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL)
        .bind(user_id.get())
        .execute(&mut *connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)?;
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}
pub(crate) async fn revoke_other_access_sessions(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    current_session_id: crate::domain_types::AdminSessionId,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_OTHER_ACCESS_SESSIONS_SQL)
        .bind(user_id.get())
        .bind(current_session_id.get().get())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}
pub(crate) async fn revoke_user_refresh_tokens(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}
