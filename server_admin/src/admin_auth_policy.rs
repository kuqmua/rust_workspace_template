#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{
    StdAdminFailureDelayMillis, StdAdminFailureThreshold, StdAdminRateLimitCount,
    StdAdminRateLimitWindowSeconds,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct AdminAuthPolicy {
    pub(crate) audit_export_limit: StdAdminRateLimitCount,
    pub(crate) failure_delay: StdAdminFailureDelayMillis,
    pub(crate) failure_threshold: StdAdminFailureThreshold,
    pub(crate) mutation_limit: StdAdminRateLimitCount,
    pub(crate) refresh_limit: StdAdminRateLimitCount,
    pub(crate) sign_in_ip_limit: StdAdminRateLimitCount,
    pub(crate) sign_in_limit: StdAdminRateLimitCount,
    pub(crate) audit_export_window: StdAdminRateLimitWindowSeconds,
    pub(crate) mutation_window: StdAdminRateLimitWindowSeconds,
    pub(crate) refresh_window: StdAdminRateLimitWindowSeconds,
    pub(crate) sign_in_window: StdAdminRateLimitWindowSeconds,
}
