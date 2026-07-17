#![allow(clippy::single_call_fn)] // dashboard summary owns one bounded aggregate query

pub(crate) async fn counts(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
) -> Result<
    (
        server_admin_contract::AdminOperationalCount,
        server_admin_contract::AdminOperationalCount,
    ),
    super::AdminRepositoryError,
> {
    let (active_sessions, failed_sign_ins) =
        sqlx::query_as::<_, (i64, i64)>(str_constants::SERVER_ADMIN_DASHBOARD_COUNTS_SQL)
            .fetch_one(pool.0)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    Ok((
        server_admin_contract::AdminOperationalCount::from(
            u64::try_from(active_sessions)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
        server_admin_contract::AdminOperationalCount::from(
            u64::try_from(failed_sign_ins)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ),
    ))
}
