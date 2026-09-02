pub(crate) async fn update_user_password(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    admin_password_hash: &crate::admin_password_hash::AdminPasswordHash,
    admin_password_change_required: crate::admin_password_change_required::AdminPasswordChangeRequired,
) -> Result<server_admin_core::std_admin_bool::StdAdminBool, crate::sqlx_admin_error::SqlxAdminError>
{
    sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_UPDATE_USER_PASSWORD_SQL)
        .bind(admin_user_record_id.get())
        .bind(admin_password_hash.expose().as_ref())
        .bind(*admin_password_change_required)
        .fetch_optional(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
}
