use crate::{AdminCleanupCfg, AdminCleanupError, AdminCleanupReport, AdminCleanupRows};

pub async fn cleanup_admin_tables(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    cfg: AdminCleanupCfg,
) -> Result<AdminCleanupReport, AdminCleanupError> {
    let access_sessions = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_ACCESS_SESSIONS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    let refresh_tokens = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_REFRESH_TOKENS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    let login_attempts = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_LOGIN_ATTEMPTS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?;
    let _audit_cleanup_permission =
        sqlx::query(constants_str::SERVER_ADMIN_ENABLE_AUDIT_CLEANUP_SQL)
            .execute(&mut *audit_tx)
            .await
            .map_err(crate::SqlxAdminError::from)
            .map_err(AdminCleanupError::Pg)?;
    let audit_log = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_AUDIT_LOG_SQL)
        .bind(cfg.audit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(&mut *audit_tx)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    audit_tx
        .commit()
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?;
    let rate_limits = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_RATE_LIMITS_SQL)
        .bind(cfg.rate_limit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
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
    .map_err(AdminCleanupError::Idempotency)?;
    let report = AdminCleanupReport::new(
        AdminCleanupRows::from(access_sessions),
        AdminCleanupRows::from(audit_log),
        AdminCleanupRows::from(u64::from(idempotency)),
        AdminCleanupRows::from(login_attempts),
        AdminCleanupRows::from(rate_limits),
        AdminCleanupRows::from(refresh_tokens),
    );
    let stored_rows =
        i64::try_from(report.total_rows().get()).map_err(|_error| AdminCleanupError::Count)?;
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_CLEANUP_STATUS_SQL)
        .bind(stored_rows)
        .execute(pool.as_ref())
        .await
        .map_err(crate::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)
        .map(drop)?;
    Ok(report)
}
