#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::DerefInner,
)]
pub struct LeaseStaleTimeoutDuration(std::time::Duration);
impl TryFrom<std::time::Duration> for LeaseStaleTimeoutDuration {
    type Error = crate::std_lease_stale_timeout_error::StdLeaseStaleTimeoutError;
    fn try_from(duration: std::time::Duration) -> Result<Self, Self::Error> {
        (!duration.is_zero())
            .then_some(Self(duration))
            .ok_or(crate::std_lease_stale_timeout_error::StdLeaseStaleTimeoutError::Zero)
    }
}
