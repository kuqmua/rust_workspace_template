#![allow(clippy::single_call_fn)] // stable root cleanup API delegates to the private bounded-cleanup module
pub(super) async fn cleanup_admin_tables(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    cfg: super::AdminCleanupCfg,
) -> Result<super::AdminCleanupReport, super::AdminCleanupError> {
    let repository_report = super::repository::cleanup::cleanup_admin_tables(
        super::repository::SqlxAdminRepositoryPoolRef::from(pool.as_ref()),
        &cfg,
    )
    .await
    .map_err(super::AdminCleanupError::Pg)?;
    let idempotency = pg_table::cleanup_pg_table_idempotency(
        pool,
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::try_from(
            cfg.idempotency_completed_retention.0,
        )?,
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::try_from(
            cfg.idempotency_pending_retention.0,
        )?,
        pg_table::PgTableIdempotencyCleanupBatchSize::try_from(cfg.batch_size.0)?,
    )
    .await
    .map_err(super::AdminCleanupError::Idempotency)?;
    let (access_sessions, audit_log, login_attempts, rate_limits, refresh_tokens) =
        repository_report.into_parts();
    let report = super::AdminCleanupReport {
        access_sessions,
        audit_log,
        idempotency: super::AdminCleanupRows::from(u64::from(idempotency)),
        login_attempts,
        rate_limits,
        refresh_tokens,
    };
    super::repository::cleanup::record_success(
        super::repository::SqlxAdminRepositoryPoolRef::from(pool.as_ref()),
        report.total_rows(),
    )
    .await?;
    Ok(report)
}
