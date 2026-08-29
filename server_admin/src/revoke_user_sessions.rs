pub(crate) async fn revoke_user_sessions(
    connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: server_admin_core::admin_user_id::AdminUserId,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL)
        .bind(user_id.get())
        .execute(&mut *connection.0)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)?;
    sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
