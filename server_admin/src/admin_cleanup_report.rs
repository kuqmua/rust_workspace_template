use super::AdminCleanupRows;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupReport {
    access_sessions: AdminCleanupRows,
    audit_log: AdminCleanupRows,
    idempotency: AdminCleanupRows,
    login_attempts: AdminCleanupRows,
    rate_limits: AdminCleanupRows,
    refresh_tokens: AdminCleanupRows,
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
