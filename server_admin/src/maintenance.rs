#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::EnumFromStr,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminAuditAction {
    Create,
    Delete,
    Refresh,
    SignIn,
    SignOut,
    Update,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::EnumFromStr,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminAuditResource {
    AuditLog,
    Permission,
    Role,
    Session,
    SystemSettings,
    User,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct SqlxAdminMigrateError(sqlx::migrate::MigrateError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
enum AdminMigrateErrorInner {
    #[error("migration failed: {0:?}")]
    Migration(SqlxAdminMigrateError),
    #[error("permission reconciliation failed: {0:?}")]
    Reconciliation(super::SqlxAdminError),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("failed to prepare administrator schema: {0}")]
#[derive(newtype::FromInner)]
pub struct AdminMigrateError(AdminMigrateErrorInner);
impl From<SqlxAdminMigrateError> for AdminMigrateError {
    fn from(error: SqlxAdminMigrateError) -> Self {
        Self(AdminMigrateErrorInner::Migration(error))
    }
}
impl From<super::SqlxAdminError> for AdminMigrateError {
    fn from(error: super::SqlxAdminError) -> Self {
        Self(AdminMigrateErrorInner::Reconciliation(error))
    }
}
pub async fn prepare_postgresql(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
) -> Result<(), AdminMigrateError> {
    crate::adapters::migrations::migrator()
        .run(pool.as_ref())
        .await
        .map_err(SqlxAdminMigrateError::from)
        .map_err(AdminMigrateError::from)?;
    let permission_names = super::AdminPermission::ALL
        .into_iter()
        .map(|permission| permission.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    let _permission_result = sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_PERMISSIONS_SQL)
        .bind(permission_names)
        .execute(pool.as_ref())
        .await
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminMigrateError::from)?;
    let _role_permission_result =
        sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_ROLE_PERMISSIONS_SQL)
            .execute(pool.as_ref())
            .await
            .map_err(super::SqlxAdminError::from)
            .map_err(AdminMigrateError::from)?;
    Ok(())
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupBatchSize(i64);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupRetentionSeconds(i64);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupCfg {
    audit_retention: AdminCleanupRetentionSeconds,
    auth_retention: AdminCleanupRetentionSeconds,
    batch_size: AdminCleanupBatchSize,
    idempotency_completed_retention: AdminCleanupRetentionSeconds,
    idempotency_pending_retention: AdminCleanupRetentionSeconds,
    rate_limit_retention: AdminCleanupRetentionSeconds,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupReport {
    access_sessions: AdminCleanupRows,
    audit_log: AdminCleanupRows,
    idempotency: AdminCleanupRows,
    login_attempts: AdminCleanupRows,
    rate_limits: AdminCleanupRows,
    refresh_tokens: AdminCleanupRows,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct AdminCleanupRows(u64);
impl std::ops::Add for AdminCleanupRows {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
impl AdminCleanupRows {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }

    fn saturating_add(self, rhs: Self) -> Self {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminCleanupCfgError {
    #[error("{}", constants_str::CLEANUP_BATCH_SIZE_MUST_BE_BETWEEN_1_AND_10000)]
    BatchSizeOutOfRange,
    #[error("{}", constants_str::CLEANUP_RETENTION_MUST_BE_GREATER_THAN_ZERO)]
    RetentionMustBePositive,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminCleanupError {
    #[error("{}", constants_str::ADMIN_CLEANUP_ROWS_EXCEED_I64)]
    Count,
    #[error("idempotency cleanup failed: {0}")]
    Idempotency(#[source] pg_table::domain_types::SqlxPgTableIdempotencyError),
    #[error(transparent)]
    IdempotencyConfig(
        #[from] pg_table::domain_types::PgTableIdempotencyCleanupValueTryFromI64Error,
    ),
    #[error("administrator table cleanup failed: {0:?}")]
    Pg(#[source] super::SqlxAdminError),
}
impl TryFrom<i64> for AdminCleanupBatchSize {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (constants_i64::ONE..=10_000i64).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminCleanupCfgError::BatchSizeOutOfRange)
        }
    }
}
impl TryFrom<i64> for AdminCleanupRetentionSeconds {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > constants_i64::ZERO {
            Ok(Self(value))
        } else {
            Err(AdminCleanupCfgError::RetentionMustBePositive)
        }
    }
}
impl AdminCleanupCfg {
    pub(crate) const fn audit_retention(self) -> AdminCleanupRetentionSeconds {
        self.audit_retention
    }

    pub(crate) const fn auth_retention(self) -> AdminCleanupRetentionSeconds {
        self.auth_retention
    }

    pub(crate) const fn batch_size(self) -> AdminCleanupBatchSize {
        self.batch_size
    }

    pub(crate) const fn idempotency_completed_retention(self) -> AdminCleanupRetentionSeconds {
        self.idempotency_completed_retention
    }

    pub(crate) const fn idempotency_pending_retention(self) -> AdminCleanupRetentionSeconds {
        self.idempotency_pending_retention
    }

    #[must_use]
    pub const fn new(
        batch_size: AdminCleanupBatchSize,
        auth_retention: AdminCleanupRetentionSeconds,
        audit_retention: AdminCleanupRetentionSeconds,
        rate_limit_retention: AdminCleanupRetentionSeconds,
        idempotency_completed_retention: AdminCleanupRetentionSeconds,
        idempotency_pending_retention: AdminCleanupRetentionSeconds,
    ) -> Self {
        Self {
            audit_retention,
            auth_retention,
            batch_size,
            idempotency_completed_retention,
            idempotency_pending_retention,
            rate_limit_retention,
        }
    }

    pub(crate) const fn rate_limit_retention(self) -> AdminCleanupRetentionSeconds {
        self.rate_limit_retention
    }
}
impl AdminCleanupBatchSize {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
impl AdminCleanupRetentionSeconds {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
impl AdminCleanupReport {
    #[allow(
        clippy::single_call_fn,
        reason = "cleanup adapter constructs the complete typed report through one invariant boundary"
    )]
    pub(crate) const fn new(
        access_sessions: AdminCleanupRows,
        audit_log: AdminCleanupRows,
        idempotency: AdminCleanupRows,
        login_attempts: AdminCleanupRows,
        rate_limits: AdminCleanupRows,
        refresh_tokens: AdminCleanupRows,
    ) -> Self {
        Self {
            access_sessions,
            audit_log,
            idempotency,
            login_attempts,
            rate_limits,
            refresh_tokens,
        }
    }

    #[must_use]
    pub fn total_rows(self) -> AdminCleanupRows {
        self.access_sessions
            .saturating_add(self.audit_log)
            .saturating_add(self.idempotency)
            .saturating_add(self.login_attempts)
            .saturating_add(self.rate_limits)
            .saturating_add(self.refresh_tokens)
    }
}
pub async fn cleanup_admin_tables(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    cfg: AdminCleanupCfg,
) -> Result<AdminCleanupReport, AdminCleanupError> {
    let access_sessions = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_ACCESS_SESSIONS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    let refresh_tokens = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_REFRESH_TOKENS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    let login_attempts = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_LOGIN_ATTEMPTS_SQL)
        .bind(cfg.auth_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    let mut audit_tx = sqlx::Acquire::begin(pool.as_ref())
        .await
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?;
    let _audit_cleanup_permission =
        sqlx::query(constants_str::SERVER_ADMIN_ENABLE_AUDIT_CLEANUP_SQL)
            .execute(&mut *audit_tx)
            .await
            .map_err(super::SqlxAdminError::from)
            .map_err(AdminCleanupError::Pg)?;
    let audit_log = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_AUDIT_LOG_SQL)
        .bind(cfg.audit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(&mut *audit_tx)
        .await
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?
        .rows_affected();
    audit_tx
        .commit()
        .await
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)?;
    let rate_limits = sqlx::query(constants_str::SERVER_ADMIN_CLEANUP_RATE_LIMITS_SQL)
        .bind(cfg.rate_limit_retention().get())
        .bind(cfg.batch_size().get())
        .execute(pool.as_ref())
        .await
        .map_err(super::SqlxAdminError::from)
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
        .map_err(super::SqlxAdminError::from)
        .map_err(AdminCleanupError::Pg)
        .map(drop)?;
    Ok(report)
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum InitialAdministratorCreationError {
    #[error("initial administrator creation audit details are invalid")]
    AuditDetails,
    #[error("initial administrator creation display name is empty")]
    EmptyDisplayName,
    #[error("initial administrator creation login has an invalid format")]
    InvalidLogin,
    #[error("initial administrator creation password does not satisfy policy")]
    InvalidPassword,
    #[error("initial administrator creation has already been completed")]
    AlreadyInitialized,
    #[error("initial administrator creation password hashing failed: {0}")]
    PasswordHash(super::AdminPasswordHashError),
    #[error("initial administrator creation database operation failed: {0:?}")]
    Pg(super::SqlxAdminError),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminPasswordResetError {
    #[error("administrator password reset audit details are invalid")]
    AuditDetails,
    #[error("administrator password reset login has an invalid format")]
    InvalidLogin,
    #[error("administrator password reset password does not satisfy policy")]
    InvalidPassword,
    #[error("administrator password reset password hashing failed: {0}")]
    PasswordHash(super::AdminPasswordHashError),
    #[error("administrator password reset database operation failed: {0:?}")]
    Pg(super::SqlxAdminError),
    #[error("administrator password reset target does not exist")]
    UnknownLogin,
}
pub async fn create_initial_administrator(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: super::AdminLogin,
    display_name: super::AdminDisplayName,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &super::AdminPasswordHasher,
) -> Result<super::AdminUserId, InitialAdministratorCreationError> {
    crate::adapters::migrations::create_initial_administrator(
        pool,
        login,
        display_name,
        password,
        password_hasher,
    )
    .await
}
pub async fn reset_admin_password(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: super::AdminLogin,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &super::AdminPasswordHasher,
) -> Result<super::AdminUserId, AdminPasswordResetError> {
    crate::adapters::migrations::reset_admin_password(pool, login, password, password_hasher).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn cleanup_batch_rejects_zero() {
        assert_eq!(
            super::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
            Err(super::AdminCleanupCfgError::BatchSizeOutOfRange),
        );
    }
}
