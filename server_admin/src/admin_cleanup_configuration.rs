#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct AdminCleanupConfiguration {
    #[getters(copy)]
    audit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    auth_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    batch_size: crate::admin_cleanup_batch_size::AdminCleanupBatchSize,
    #[getters(copy)]
    idempotency_completed_retention:
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    idempotency_pending_retention:
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    rate_limit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
}
impl AdminCleanupConfiguration {
    #[must_use]
    pub const fn new(
        admin_cleanup_batch_size: crate::admin_cleanup_batch_size::AdminCleanupBatchSize,
        auth_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        audit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        rate_limit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        idempotency_completed_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
        idempotency_pending_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    ) -> Self {
        Self {
            audit_retention,
            auth_retention,
            batch_size: admin_cleanup_batch_size,
            idempotency_completed_retention,
            idempotency_pending_retention,
            rate_limit_retention,
        }
    }
}
