#[derive(
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
#[getters(bare, legacy_refs)]
pub struct AdminCleanupReport {
    #[getters(copy)]
    access_sessions: crate::admin_cleanup_rows::AdminCleanupRows,
    #[getters(copy)]
    audit_log: crate::admin_cleanup_rows::AdminCleanupRows,
    #[getters(copy)]
    idempotency: crate::admin_cleanup_rows::AdminCleanupRows,
    #[getters(copy)]
    login_attempts: crate::admin_cleanup_rows::AdminCleanupRows,
    #[getters(copy)]
    rate_limits: crate::admin_cleanup_rows::AdminCleanupRows,
    #[getters(copy)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_cleanup_report_copy_accessors_preserve_reference_compatibility() {
        let [
            access_sessions,
            audit_log,
            idempotency,
            login_attempts,
            rate_limits,
            refresh_tokens,
        ] = [1u64, 2u64, 3u64, 4u64, 5u64, 6u64]
            .map(crate::admin_cleanup_rows::AdminCleanupRows::from);
        let admin_cleanup_report = super::AdminCleanupReport::new(
            access_sessions,
            audit_log,
            idempotency,
            login_attempts,
            rate_limits,
            refresh_tokens,
        );
        assert_eq!(
            [
                admin_cleanup_report.access_sessions(),
                admin_cleanup_report.audit_log(),
                admin_cleanup_report.idempotency(),
                admin_cleanup_report.login_attempts(),
                admin_cleanup_report.rate_limits(),
                admin_cleanup_report.refresh_tokens()
            ],
            [
                access_sessions,
                audit_log,
                idempotency,
                login_attempts,
                rate_limits,
                refresh_tokens
            ],
        );
        assert_eq!(
            [
                admin_cleanup_report.get_access_sessions(),
                admin_cleanup_report.get_audit_log(),
                admin_cleanup_report.get_idempotency(),
                admin_cleanup_report.get_login_attempts(),
                admin_cleanup_report.get_rate_limits(),
                admin_cleanup_report.get_refresh_tokens()
            ],
            [
                &access_sessions,
                &audit_log,
                &idempotency,
                &login_attempts,
                &rate_limits,
                &refresh_tokens
            ],
        );
        assert_eq!(
            admin_cleanup_report.total_rows(),
            crate::admin_cleanup_rows::AdminCleanupRows::from(21u64)
        );
    }
}
