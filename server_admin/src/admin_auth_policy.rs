#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct AdminAuthPolicy {
    pub(crate) audit_export_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    pub(crate) failure_delay: crate::std_admin_failure_delay_millis::StdAdminFailureDelayMillis,
    pub(crate) failure_threshold: crate::std_admin_failure_threshold::StdAdminFailureThreshold,
    pub(crate) mutation_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    pub(crate) refresh_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    pub(crate) sign_in_ip_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    pub(crate) sign_in_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    pub(crate) audit_export_window:
        crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
    pub(crate) mutation_window:
        crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
    pub(crate) refresh_window:
        crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
    pub(crate) sign_in_window:
        crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
}
