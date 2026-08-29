pub(crate) async fn revoke_access_session(
    connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    session_id: crate::admin_session_id::AdminSessionId,
    user_id: server_admin_core::admin_user_id::AdminUserId,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_REVOKE_ACCESS_SESSION_SQL)
        .bind(session_id.get().get())
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
