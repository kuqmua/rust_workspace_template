#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunIntervalDuration(std::time::Duration);

impl RunIntervalDuration {
    pub(crate) const fn get(self) -> std::time::Duration {
        self.0
    }
}

impl TryFrom<std::time::Duration> for RunIntervalDuration {
    type Error =
        crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        (!value.is_zero()).then_some(Self(value)).ok_or(
            crate::std_run_interval_try_from_duration_error::StdRunIntervalTryFromDurationError::Zero,
        )
    }
}
