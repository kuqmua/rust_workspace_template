pub(crate) async fn read_last_admin_state(
    mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
    admin_user_record_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
) -> Result<crate::last_admin_state::LastAdminState, crate::sqlx_admin_error::SqlxAdminError> {
    let target_is_admin =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USER_IS_ADMIN_SQL)
            .bind(admin_user_record_id.get())
            .fetch_one(&mut **sqlx_admin_repository_connection_mut_ref)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let active_count =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL)
            .fetch_one(&mut **sqlx_admin_repository_connection_mut_ref)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    Ok(crate::last_admin_state::LastAdminState::new(
        crate::admin_active_administrator_count::AdminActiveAdministratorCount::from(active_count),
        server_admin_core::std_admin_bool::StdAdminBool::from(target_is_admin),
    ))
}
