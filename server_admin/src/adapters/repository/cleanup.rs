#![allow(clippy::single_call_fn)] // the cleanup transaction owns all related retention queries

pub(crate) async fn cleanup_admin_tables(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    cfg: &crate::domain_types::AdminCleanupCfg,
) -> Result<super::AdminCleanupRepositoryReport, crate::domain_types::SqlxAdminError> {
    let access_sessions = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_ACCESS_SESSIONS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .rows_affected();
    let refresh_tokens = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_REFRESH_TOKENS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .rows_affected();
    let login_attempts = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_LOGIN_ATTEMPTS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let _audit_cleanup_permission =
        sqlx::query(constants_str::SERVER_ADMIN_ENABLE_AUDIT_CLEANUP_SQL)
            .execute(&mut *audit_tx)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
    let audit_log = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_AUDIT_LOG_SQL)
        .bind(cfg.audit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(&mut *audit_tx)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .rows_affected();
    audit_tx
        .commit()
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?;
    let rate_limits = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_RATE_LIMITS_SQL)
        .bind(cfg.rate_limit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)?
        .rows_affected();
    Ok(super::AdminCleanupRepositoryReport {
        access_sessions: crate::domain_types::AdminCleanupRows::from(access_sessions),
        audit_log: crate::domain_types::AdminCleanupRows::from(audit_log),
        login_attempts: crate::domain_types::AdminCleanupRows::from(login_attempts),
        rate_limits: crate::domain_types::AdminCleanupRows::from(rate_limits),
        refresh_tokens: crate::domain_types::AdminCleanupRows::from(refresh_tokens),
    })
}

pub(crate) async fn record_success(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    rows: crate::domain_types::AdminCleanupRows,
) -> Result<(), crate::domain_types::AdminCleanupError> {
    let stored_rows = i64::try_from(rows.get())
        .map_err(|_error| crate::domain_types::AdminCleanupError::Count)?;
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_CLEANUP_STATUS_SQL)
        .bind(stored_rows)
        .execute(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map_err(crate::domain_types::AdminCleanupError::Pg)
        .map(drop)
}
