pub(crate) async fn revoke_refresh_token(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_token_hash: &crate::admin_token_hash::AdminTokenHash,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    let query = sqlx::query(constants_str::SERVER_ADMIN_REVOKE_REFRESH_TOKEN_SQL);
    query
        .bind(admin_token_hash.expose().as_ref())
        .bind(admin_user_record_id.get())
        .execute(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
