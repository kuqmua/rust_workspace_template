pub(crate) async fn insert_user(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_login: &server_admin_contract::admin_login::AdminLogin,
    admin_display_name: &server_admin_contract::admin_display_name::AdminDisplayName,
    admin_password_hash: &crate::admin_password_hash::AdminPasswordHash,
) -> Result<
    server_admin_core::admin_user_record_id::AdminUserRecordId,
    crate::sqlx_admin_error::SqlxAdminError,
> {
    sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(admin_login.as_ref())
        .bind(admin_display_name.as_ref())
        .bind(admin_password_hash.expose().as_ref())
        .fetch_one(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .and_then(|value| {
            server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(value)
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        })
}
