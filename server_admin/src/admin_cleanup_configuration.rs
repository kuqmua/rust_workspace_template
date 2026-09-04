#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct AdminCleanupConfiguration {
    #[getters(copy)]
    #[constructor(order = 2)]
    audit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    #[constructor(order = 1)]
    auth_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    #[constructor(order = 0)]
    batch_size: crate::admin_cleanup_batch_size::AdminCleanupBatchSize,
    #[getters(copy)]
    #[constructor(order = 4)]
    idempotency_completed_retention:
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    #[constructor(order = 5)]
    idempotency_pending_retention:
        crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
    #[getters(copy)]
    #[constructor(order = 3)]
    rate_limit_retention: crate::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds,
}
