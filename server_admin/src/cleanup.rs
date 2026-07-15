#![allow(clippy::single_call_fn)] // stable root cleanup API delegates to the private bounded-cleanup module
pub(super) async fn cleanup_admin_tables(
    pool: app_state::SqlxPgPoolRef<'_>,
    cfg: super::AdminCleanupCfg,
) -> Result<super::AdminCleanupReport, super::AdminCleanupError> {
    let access_sessions = sqlx::query(str_constants::expr::S_0834)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.as_ref())
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?
        .rows_affected();
    let refresh_tokens = sqlx::query(str_constants::expr::S_0837)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.as_ref())
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?
        .rows_affected();
    let login_attempts = sqlx::query(str_constants::expr::S_0836)
        .bind(cfg.auth_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.as_ref())
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?
        .rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.as_ref())
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?;
    let _audit_cleanup_permission = sqlx::query(str_constants::expr::S_0780)
        .execute(&mut *audit_tx)
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?;
    let audit_log = sqlx::query(str_constants::expr::S_0835)
        .bind(cfg.audit_retention.0)
        .bind(cfg.batch_size.0)
        .execute(&mut *audit_tx)
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?
        .rows_affected();
    audit_tx
        .commit()
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?;
    let rate_limits = sqlx::query(str_constants::expr::S_0838)
        .bind(cfg.rate_limit_retention.0)
        .bind(cfg.batch_size.0)
        .execute(pool.as_ref())
        .await
        .map_err(|error| super::AdminCleanupError::Pg(super::SqlxAdminError::from(error)))?
        .rows_affected();
    let idempotency = pg_table::cleanup_pg_table_idempotency(
        pool,
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::from(
            cfg.idempotency_completed_retention.0,
        ),
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::from(
            cfg.idempotency_pending_retention.0,
        ),
        pg_table::PgTableIdempotencyCleanupBatchSize::from(cfg.batch_size.0),
    )
    .await
    .map_err(super::AdminCleanupError::Idempotency)?;
    Ok(super::AdminCleanupReport {
        access_sessions: super::AdminCleanupRows::from(access_sessions),
        audit_log: super::AdminCleanupRows::from(audit_log),
        idempotency: super::AdminCleanupRows::from(u64::from(idempotency)),
        login_attempts: super::AdminCleanupRows::from(login_attempts),
        rate_limits: super::AdminCleanupRows::from(rate_limits),
        refresh_tokens: super::AdminCleanupRows::from(refresh_tokens),
    })
}
