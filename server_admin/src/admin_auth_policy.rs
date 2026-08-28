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
    pub(super) audit_export_limit: StdAdminRateLimitCount,
    pub(super) failure_delay: StdAdminFailureDelayMillis,
    pub(super) failure_threshold: StdAdminFailureThreshold,
    pub(super) mutation_limit: StdAdminRateLimitCount,
    pub(super) refresh_limit: StdAdminRateLimitCount,
    pub(super) sign_in_ip_limit: StdAdminRateLimitCount,
    pub(super) sign_in_limit: StdAdminRateLimitCount,
    pub(super) audit_export_window: StdAdminRateLimitWindowSeconds,
    pub(super) mutation_window: StdAdminRateLimitWindowSeconds,
    pub(super) refresh_window: StdAdminRateLimitWindowSeconds,
    pub(super) sign_in_window: StdAdminRateLimitWindowSeconds,
}
