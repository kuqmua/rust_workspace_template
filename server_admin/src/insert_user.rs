pub(crate) async fn insert_user(
    mut connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    login: &server_admin_contract::admin_login::AdminLogin,
    display_name: &server_admin_contract::admin_display_name::AdminDisplayName,
    password_hash: &crate::admin_password_hash::AdminPasswordHash,
) -> Result<
    server_admin_core::admin_user_record_id::AdminUserRecordId,
    crate::sqlx_admin_error::SqlxAdminError,
> {
    sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.expose().as_ref())
        .fetch_one(&mut **connection)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .and_then(|value| {
            server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(value)
                .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        })
}
