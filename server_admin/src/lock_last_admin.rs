// The owner module retains lint-sensitive semantics from the original implementation.

pub(crate) async fn lock_last_admin(
    mut connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(&mut **connection)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
