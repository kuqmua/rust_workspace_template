pub(crate) async fn insert_user(
    connection: crate::SqlxAdminRepositoryConnectionMutRef<'_>,
    login: &server_admin_contract::domain_types::AdminLogin,
    display_name: &server_admin_contract::domain_types::AdminDisplayName,
    password_hash: &crate::domain_types::AdminPasswordHash,
) -> Result<crate::domain_types::AdminUserId, crate::domain_types::SqlxAdminError> {
    sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_INSERT_USER_SQL)
        .bind(login.as_ref())
        .bind(display_name.as_ref())
        .bind(password_hash.expose().as_ref())
        .fetch_one(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .and_then(|value| {
            crate::domain_types::AdminUserId::try_from(value)
                .map_err(crate::domain_types::SqlxAdminError::from)
        })
}
