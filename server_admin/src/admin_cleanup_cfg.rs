#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
)]
pub struct AdminCleanupCfg {
    audit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    auth_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    batch_size: crate::admin_cleanup_batch_size::AdminCleanupBatchSize,
    idempotency_completed_retention:
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    idempotency_pending_retention:
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    rate_limit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
}
impl AdminCleanupCfg {
    pub(crate) const fn audit_retention(
        self,
    ) -> crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds {
        self.audit_retention
    }

    pub(crate) const fn auth_retention(
        self,
    ) -> crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds {
        self.auth_retention
    }

    pub(crate) const fn batch_size(self) -> crate::admin_cleanup_batch_size::AdminCleanupBatchSize {
        self.batch_size
    }

    pub(crate) const fn idempotency_completed_retention(
        self,
    ) -> crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds {
        self.idempotency_completed_retention
    }

    pub(crate) const fn idempotency_pending_retention(
        self,
    ) -> crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds {
        self.idempotency_pending_retention
    }

    #[must_use]
    pub const fn new(
        batch_size: crate::admin_cleanup_batch_size::AdminCleanupBatchSize,
        auth_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        audit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        rate_limit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        idempotency_completed_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        idempotency_pending_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
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

    pub(crate) const fn rate_limit_retention(
        self,
    ) -> crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds {
        self.rate_limit_retention
    }
}
