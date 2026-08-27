// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)]
pub(crate) async fn read_last_admin_state(
    connection: super::super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
) -> Result<super::LastAdminState, crate::domain_types::SqlxAdminError> {
    let target_is_admin =
        sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USER_IS_ADMIN_SQL)
            .bind(user_id.get())
            .fetch_one(&mut *connection.0)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
    let active_count =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL)
            .fetch_one(connection.0)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
    Ok(super::LastAdminState {
        active_count: super::AdminActiveAdministratorCount::from(active_count),
        target_is_admin: crate::domain_types::StdAdminBool::from(target_is_admin),
    })
}
