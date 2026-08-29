pub(crate) async fn update_user_password(
    connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: server_admin_core::admin_user_id::AdminUserId,
    password_hash: &crate::admin_password_hash::AdminPasswordHash,
    password_change_required: crate::admin_password_change_required::AdminPasswordChangeRequired,
) -> Result<server_admin_core::std_admin_bool::StdAdminBool, crate::sqlx_admin_error::SqlxAdminError>
{
    sqlx::query_scalar::<_, bool>(
        constants_str::integration_fixtures::SERVER_ADMIN_UPDATE_USER_PASSWORD_SQL,
    )
    .bind(user_id.get())
    .bind(password_hash.expose().as_ref())
    .bind(*password_change_required)
    .fetch_optional(connection.0)
    .await
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map(|value| server_admin_core::std_admin_bool::StdAdminBool::from(value.is_some()))
}
