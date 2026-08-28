pub(crate) async fn revoke_user_sessions(
    connection: crate::SqlxAdminRepositoryConnectionMutRef<'_>,
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
