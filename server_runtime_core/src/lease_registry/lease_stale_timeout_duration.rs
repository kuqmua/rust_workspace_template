use super::StdLeaseStaleTimeoutError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeaseStaleTimeoutDuration(pub(super) std::time::Duration);
impl TryFrom<std::time::Duration> for LeaseStaleTimeoutDuration {
    type Error = StdLeaseStaleTimeoutError;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdLeaseStaleTimeoutError)
        } else {
            Ok(Self(value))
        }
    }
}
