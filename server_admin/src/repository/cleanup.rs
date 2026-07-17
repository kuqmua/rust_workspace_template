#![allow(clippy::single_call_fn)] // the cleanup transaction owns all related retention queries

const CLEANUP_ACCESS_SESSIONS: &str = "WITH expired AS (SELECT id FROM admin_access_sessions WHERE expires_at < now() OR (revoked_at IS NOT NULL AND revoked_at < now() - make_interval(secs => $1)) ORDER BY expires_at LIMIT $2) DELETE FROM admin_access_sessions target USING expired WHERE target.id=expired.id";
const CLEANUP_REFRESH_TOKENS: &str = "WITH expired AS (SELECT id FROM admin_refresh_tokens WHERE expires_at < now() OR (revoked_at IS NOT NULL AND revoked_at < now() - make_interval(secs => $1)) ORDER BY expires_at LIMIT $2) DELETE FROM admin_refresh_tokens target USING expired WHERE target.id=expired.id";
const CLEANUP_LOGIN_ATTEMPTS: &str = "WITH expired AS (SELECT id FROM admin_login_attempts WHERE attempted_at < now() - make_interval(secs => $1) ORDER BY attempted_at LIMIT $2) DELETE FROM admin_login_attempts target USING expired WHERE target.id=expired.id";
const ENABLE_AUDIT_CLEANUP: &str = "SET LOCAL app.admin_audit_cleanup = 'on'";
const CLEANUP_AUDIT_LOG: &str = "WITH expired AS (SELECT id FROM admin_audit_log WHERE created_at < now() - make_interval(secs => $1) ORDER BY created_at LIMIT $2) DELETE FROM admin_audit_log target USING expired WHERE target.id=expired.id";
const CLEANUP_RATE_LIMITS: &str = "WITH expired AS (SELECT scope,subject FROM admin_rate_limits WHERE window_started_at < now() - make_interval(secs => $1) ORDER BY window_started_at LIMIT $2) DELETE FROM admin_rate_limits target USING expired WHERE target.scope=expired.scope AND target.subject=expired.subject";

pub(crate) async fn cleanup_admin_tables(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    cfg: &crate::AdminCleanupCfg,
) -> Result<super::AdminCleanupRepositoryReport, crate::SqlxAdminError> {
    let access_sessions = sqlx::query(CLEANUP_ACCESS_SESSIONS)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    let refresh_tokens = sqlx::query(CLEANUP_REFRESH_TOKENS)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    let login_attempts = sqlx::query(CLEANUP_LOGIN_ATTEMPTS)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?
        .rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let _audit_cleanup_permission = sqlx::query(ENABLE_AUDIT_CLEANUP)
        .execute(&mut *audit_tx)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let audit_log = sqlx::query(CLEANUP_AUDIT_LOG)
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
    let rate_limits = sqlx::query(CLEANUP_RATE_LIMITS)
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
