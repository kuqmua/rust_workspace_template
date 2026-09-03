pub(crate) async fn lock_last_admin(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_LOCK_LAST_ADMIN_SQL)
        .execute(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
