#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct RetryPolicy {
    attempts: crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize,
    delay: Option<crate::retry_delay_duration::RetryDelayDuration>,
}

impl RetryPolicy {
    #[must_use]
    pub const fn attempts(self) -> crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize {
        self.attempts
    }

    #[must_use]
    pub const fn delay(self) -> Option<crate::retry_delay_duration::RetryDelayDuration> {
        self.delay
    }
}
