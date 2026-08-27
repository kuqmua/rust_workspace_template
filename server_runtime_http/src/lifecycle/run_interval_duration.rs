#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunIntervalDuration(pub(super) std::time::Duration);

impl TryFrom<std::time::Duration> for RunIntervalDuration {
    type Error = super::StdRunIntervalTryFromDurationError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(super::StdRunIntervalTryFromDurationError)
        } else {
            Ok(Self(value))
        }
    }
}
