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
    attempts: super::RetryAttemptsNonZeroUsize,
    delay: Option<super::RetryDelayDuration>,
}

impl RetryPolicy {
    #[must_use]
    pub const fn attempts(self) -> super::RetryAttemptsNonZeroUsize {
        self.attempts
    }

    #[must_use]
    pub const fn delay(self) -> Option<super::RetryDelayDuration> {
        self.delay
    }
}
