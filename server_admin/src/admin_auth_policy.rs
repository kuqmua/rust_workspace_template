#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    generate_accessor::Getters,
)]
pub struct AdminAuthPolicy {
    audit_export_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    failure_delay: crate::std_admin_failure_delay_millis::StdAdminFailureDelayMillis,
    failure_threshold: crate::std_admin_failure_threshold::StdAdminFailureThreshold,
    mutation_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    refresh_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    sign_in_ip_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    sign_in_limit: crate::std_admin_rate_limit_count::StdAdminRateLimitCount,
    audit_export_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
    mutation_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
    refresh_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
    sign_in_window: crate::std_admin_rate_limit_window_seconds::StdAdminRateLimitWindowSeconds,
}
