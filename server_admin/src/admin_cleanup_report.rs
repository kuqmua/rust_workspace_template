#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
)]
pub struct AdminCleanupReport {
    access_sessions: crate::admin_cleanup_rows::AdminCleanupRows,
    audit_log: crate::admin_cleanup_rows::AdminCleanupRows,
    idempotency: crate::admin_cleanup_rows::AdminCleanupRows,
    login_attempts: crate::admin_cleanup_rows::AdminCleanupRows,
    rate_limits: crate::admin_cleanup_rows::AdminCleanupRows,
    refresh_tokens: crate::admin_cleanup_rows::AdminCleanupRows,
}
impl AdminCleanupReport {
    #[must_use]
    pub fn total_rows(self) -> crate::admin_cleanup_rows::AdminCleanupRows {
        self.access_sessions
            .saturating_add(self.audit_log)
            .saturating_add(self.idempotency)
            .saturating_add(self.login_attempts)
            .saturating_add(self.rate_limits)
            .saturating_add(self.refresh_tokens)
    }
}
