pub(crate) async fn revoke_refresh_token(
    connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    token_hash: &crate::admin_token_hash::AdminTokenHash,
    user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_REVOKE_REFRESH_TOKEN_SQL)
        .bind(token_hash.expose().as_ref())
        .bind(user_id.get())
        .execute(connection.0)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
