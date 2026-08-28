pub(crate) async fn revoke_access_session(
    connection: crate::SqlxAdminRepositoryConnectionMutRef<'_>,
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
