pub(crate) async fn revoke_access_session(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_session_id: crate::admin_session_id::AdminSessionId,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
) -> Result<(), crate::sqlx_admin_error::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_ACCESS_SESSION_SQL)
        .bind(admin_session_id.get().get())
        .bind(admin_user_record_id.get())
        .execute(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(drop)
}
