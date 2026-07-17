#![allow(clippy::single_call_fn)] // the cleanup transaction owns all related retention queries

pub(crate) async fn cleanup_admin_tables(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    cfg: &crate::AdminCleanupCfg,
) -> Result<super::AdminCleanupRepositoryReport, crate::SqlxAdminError> {
    let access_sessions = sqlx::query(str_constants::SERVER_ADMIN_CLEANUP_ACCESS_SESSIONS_SQL)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    let refresh_tokens = sqlx::query(str_constants::SERVER_ADMIN_CLEANUP_REFRESH_TOKENS_SQL)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    let login_attempts = sqlx::query(str_constants::SERVER_ADMIN_CLEANUP_LOGIN_ATTEMPTS_SQL)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let _audit_cleanup_permission =
        sqlx::query(str_constants::SERVER_ADMIN_ENABLE_AUDIT_CLEANUP_SQL)
            .execute(&mut *audit_tx)
            .await
            .map_err(crate::SqlxAdminError::from)?;
    let audit_log = sqlx::query(str_constants::SERVER_ADMIN_CLEANUP_AUDIT_LOG_SQL)
        .bind(cfg.audit_retention.0)
        .bind(cfg.batch_size.0)
        .execute(&mut *audit_tx)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    audit_tx
        .commit()
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let rate_limits = sqlx::query(str_constants::SERVER_ADMIN_CLEANUP_RATE_LIMITS_SQL)
        .bind(cfg.rate_limit_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    Ok(super::AdminCleanupRepositoryReport {
        access_sessions: crate::AdminCleanupRows::from(access_sessions),
        audit_log: crate::AdminCleanupRows::from(audit_log),
        login_attempts: crate::AdminCleanupRows::from(login_attempts),
        rate_limits: crate::AdminCleanupRows::from(rate_limits),
        refresh_tokens: crate::AdminCleanupRows::from(refresh_tokens),
    })
}
