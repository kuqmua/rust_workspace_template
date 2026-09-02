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
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        (!value.is_zero())
            .then_some(Self(value))
            .ok_or(crate::std_lease_stale_timeout_error::StdLeaseStaleTimeoutError::Zero)
    }
}
