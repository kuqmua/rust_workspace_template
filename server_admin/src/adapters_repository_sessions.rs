#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

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
