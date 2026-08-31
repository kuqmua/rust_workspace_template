// The owner module retains lint-sensitive semantics from the original implementation.

pub(crate) async fn read_last_admin_state(
    connection: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
) -> Result<crate::last_admin_state::LastAdminState, crate::sqlx_admin_error::SqlxAdminError> {
    let target_is_admin =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USER_IS_ADMIN_SQL)
            .bind(user_id.get())
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let active_count =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL)
            .fetch_one(connection.0)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    Ok(crate::last_admin_state::LastAdminState {
        active_count: crate::admin_active_administrator_count::AdminActiveAdministratorCount::from(
            active_count,
        ),
        target_is_admin: server_admin_core::std_admin_bool::StdAdminBool::from(target_is_admin),
    })
}
