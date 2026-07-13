#![allow(clippy::single_call_fn)] // stable root cleanup API delegates to the private bounded-cleanup module
pub(super) async fn cleanup_admin_tables(
    pool: app_state::SqlxPgPoolRef<'_>,
    cfg: super::AdminCleanupCfg,
) -> Result<super::AdminCleanupReport, super::AdminCleanupEr> {
    let access_sessions = sqlx::query("WITH expired AS (SELECT id FROM admin_access_sessions WHERE expires_at < NOW() OR (revoked_at IS NOT NULL AND revoked_at < NOW() - make_interval(secs => $1)) ORDER BY expires_at LIMIT $2) DELETE FROM admin_access_sessions target USING expired WHERE target.id=expired.id")
        .bind(cfg.auth_retention.0).bind(cfg.batch_size.0).execute(pool.as_ref()).await.map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?.rows_affected();
    let refresh_tokens = sqlx::query("WITH expired AS (SELECT id FROM admin_refresh_tokens WHERE expires_at < NOW() OR (revoked_at IS NOT NULL AND revoked_at < NOW() - make_interval(secs => $1)) ORDER BY expires_at LIMIT $2) DELETE FROM admin_refresh_tokens target USING expired WHERE target.id=expired.id")
        .bind(cfg.auth_retention.0).bind(cfg.batch_size.0).execute(pool.as_ref()).await.map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?.rows_affected();
    let login_attempts = sqlx::query("WITH expired AS (SELECT id FROM admin_login_attempts WHERE attempted_at < NOW() - make_interval(secs => $1) ORDER BY attempted_at LIMIT $2) DELETE FROM admin_login_attempts target USING expired WHERE target.id=expired.id")
        .bind(cfg.auth_retention.0).bind(cfg.batch_size.0).execute(pool.as_ref()).await.map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?.rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.as_ref())
        .await
        .map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?;
    let _audit_cleanup_permission = sqlx::query("SET LOCAL app.admin_audit_cleanup = 'on'")
        .execute(&mut *audit_tx)
        .await
        .map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?;
    let audit_log = sqlx::query("WITH expired AS (SELECT id FROM admin_audit_log WHERE created_at < NOW() - make_interval(secs => $1) ORDER BY created_at LIMIT $2) DELETE FROM admin_audit_log target USING expired WHERE target.id=expired.id")
        .bind(cfg.audit_retention.0).bind(cfg.batch_size.0).execute(&mut *audit_tx).await.map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?.rows_affected();
    audit_tx
        .commit()
        .await
        .map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?;
    let rate_limits = sqlx::query("WITH expired AS (SELECT scope,subject FROM admin_rate_limits WHERE window_started_at < NOW() - make_interval(secs => $1) ORDER BY window_started_at LIMIT $2) DELETE FROM admin_rate_limits target USING expired WHERE target.scope=expired.scope AND target.subject=expired.subject")
        .bind(cfg.rate_limit_retention.0).bind(cfg.batch_size.0).execute(pool.as_ref()).await.map_err(|error| super::AdminCleanupEr::Pg(super::SqlxAdminEr::from(error)))?.rows_affected();
    let idempotency = pg_tbl::cleanup_pg_tbl_idempotency(
        pool,
        pg_tbl::PgTblIdempotencyCleanupRetentionSeconds::from(
            cfg.idempotency_completed_retention.0,
        ),
        pg_tbl::PgTblIdempotencyCleanupRetentionSeconds::from(cfg.idempotency_pending_retention.0),
        pg_tbl::PgTblIdempotencyCleanupBatchSize::from(cfg.batch_size.0),
    )
    .await
    .map_err(super::AdminCleanupEr::Idempotency)?;
    Ok(super::AdminCleanupReport {
        access_sessions: super::AdminCleanupRows::from(access_sessions),
        audit_log: super::AdminCleanupRows::from(audit_log),
        idempotency: super::AdminCleanupRows::from(u64::from(idempotency)),
        login_attempts: super::AdminCleanupRows::from(login_attempts),
        rate_limits: super::AdminCleanupRows::from(rate_limits),
        refresh_tokens: super::AdminCleanupRows::from(refresh_tokens),
    })
}
