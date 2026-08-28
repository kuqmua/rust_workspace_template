// The owner module retains lint-sensitive semantics from the original implementation.

pub(crate) async fn lock_last_admin(
    connection: crate::SqlxAdminRepositoryConnectionMutRef<'_>,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}
