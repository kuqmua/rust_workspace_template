use super::{AdminCleanupBatchSize, AdminCleanupRetentionSeconds};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupCfg {
    audit_retention: AdminCleanupRetentionSeconds,
    auth_retention: AdminCleanupRetentionSeconds,
    batch_size: AdminCleanupBatchSize,
    idempotency_completed_retention: AdminCleanupRetentionSeconds,
    idempotency_pending_retention: AdminCleanupRetentionSeconds,
    rate_limit_retention: AdminCleanupRetentionSeconds,
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
