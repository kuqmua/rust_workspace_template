#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeaseStaleTimeoutDuration(pub(super) std::time::Duration);
impl TryFrom<std::time::Duration> for LeaseStaleTimeoutDuration {
    type Error = crate::std_lease_stale_timeout_error::StdLeaseStaleTimeoutError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(crate::std_lease_stale_timeout_error::StdLeaseStaleTimeoutError)
        } else {
            Ok(Self(value))
        }
    }
}
