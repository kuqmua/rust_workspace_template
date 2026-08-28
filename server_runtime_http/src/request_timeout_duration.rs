#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestTimeoutDuration(std::time::Duration);

impl RequestTimeoutDuration {
    pub(crate) const fn get(self) -> std::time::Duration {
        self.0
    }
}

impl TryFrom<std::time::Duration> for RequestTimeoutDuration {
    type Error = super::StdRequestTimeoutTryFromDurationError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(super::StdRequestTimeoutTryFromDurationError)
        } else {
            Ok(Self(value))
        }
    }
}
