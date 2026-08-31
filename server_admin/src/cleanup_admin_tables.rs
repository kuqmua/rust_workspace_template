pub async fn cleanup_admin_tables(
    pool: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    cfg: crate::admin_cleanup_cfg::AdminCleanupCfg,
) -> Result<
    crate::admin_cleanup_report::AdminCleanupReport,
    crate::admin_cleanup_error::AdminCleanupError,
> {
    let access_sessions = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_ACCESS_SESSIONS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?
        .rows_affected();
    let refresh_tokens = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_REFRESH_TOKENS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?
        .rows_affected();
    let login_attempts = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_LOGIN_ATTEMPTS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?
        .rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?;
    let _audit_cleanup_permission =
        sqlx::query(constants_str::SERVER_ADMIN_ENABLE_AUDIT_CLEANUP_SQL)
            .execute(&mut *audit_tx)
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?;
    let audit_log = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_AUDIT_LOG_SQL)
        .bind(cfg.audit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(&mut *audit_tx)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?
        .rows_affected();
    audit_tx
        .commit()
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?;
    let rate_limits = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_RATE_LIMITS_SQL)
        .bind(cfg.rate_limit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)?
        .rows_affected();
    let idempotency = pg_table::cleanup_pg_table_idempotency::cleanup_pg_table_idempotency(
        pool,
        pg_table::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(
            cfg.idempotency_completed_retention().get(),
        )?,
        pg_table::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(
            cfg.idempotency_pending_retention().get(),
        )?,
        pg_table::pg_table_idempotency_cleanup_batch_size::PgTableIdempotencyCleanupBatchSize::try_from(cfg.batch_size().get())?,
    )
    .await
    .map_err(crate::admin_cleanup_error::AdminCleanupError::Idempotency)?;
    let report = crate::admin_cleanup_report::AdminCleanupReport::new(
        crate::admin_cleanup_rows::AdminCleanupRows::from(access_sessions),
        crate::admin_cleanup_rows::AdminCleanupRows::from(audit_log),
        crate::admin_cleanup_rows::AdminCleanupRows::from(u64::from(idempotency)),
        crate::admin_cleanup_rows::AdminCleanupRows::from(login_attempts),
        crate::admin_cleanup_rows::AdminCleanupRows::from(rate_limits),
        crate::admin_cleanup_rows::AdminCleanupRows::from(refresh_tokens),
    );
    let stored_rows = i64::try_from(report.total_rows().get())
        .map_err(|_error| crate::admin_cleanup_error::AdminCleanupError::Count)?;
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_CLEANUP_STATUS_SQL)
        .bind(stored_rows)
        .execute(pool.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_cleanup_error::AdminCleanupError::Pg)
        .map(drop)?;
    Ok(report)
}
