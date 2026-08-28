use crate::AdminCleanupRows;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupReport {
    pub(crate) access_sessions: AdminCleanupRows,
    pub(crate) audit_log: AdminCleanupRows,
    pub(crate) idempotency: AdminCleanupRows,
    pub(crate) login_attempts: AdminCleanupRows,
    pub(crate) rate_limits: AdminCleanupRows,
    pub(crate) refresh_tokens: AdminCleanupRows,
}
impl AdminCleanupReport {
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
