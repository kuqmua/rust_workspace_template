#![allow(clippy::single_call_fn)] // stable root cleanup API delegates to the private bounded-cleanup module
pub(crate) async fn cleanup_admin_tables(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    cfg: crate::domain_types::AdminCleanupCfg,
) -> Result<crate::domain_types::AdminCleanupReport, crate::domain_types::AdminCleanupError> {
    let repository_report = crate::adapters::repository::cleanup::cleanup_admin_tables(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(pool.as_ref()),
        &cfg,
    )
    .await
    .map_err(crate::domain_types::AdminCleanupError::Pg)?;
    let idempotency = pg_table::domain_types::cleanup_pg_table_idempotency(
        pool,
        pg_table::domain_types::PgTableIdempotencyCleanupRetentionSeconds::try_from(
            cfg.idempotency_completed_retention().get(),
        )?,
        pg_table::domain_types::PgTableIdempotencyCleanupRetentionSeconds::try_from(
            cfg.idempotency_pending_retention().get(),
        )?,
        pg_table::domain_types::PgTableIdempotencyCleanupBatchSize::try_from(
            cfg.batch_size().get(),
        )?,
    )
    .await
    .map_err(crate::domain_types::AdminCleanupError::Idempotency)?;
    let (access_sessions, audit_log, login_attempts, rate_limits, refresh_tokens) =
        repository_report.into_parts();
    let report = crate::domain_types::AdminCleanupReport::new(
        access_sessions,
        audit_log,
        crate::domain_types::AdminCleanupRows::from(u64::from(idempotency)),
        login_attempts,
        rate_limits,
        refresh_tokens,
    );
    crate::adapters::repository::cleanup::record_success(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(pool.as_ref()),
        report.total_rows(),
    )
    .await?;
    Ok(report)
}
