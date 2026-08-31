#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq, generate_constructor::New,
)]
#[constructor(pub(crate))]
pub struct RetryOutcome<Success, Error> {
    attempts: crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize,
    result: Result<Success, Error>,
}

impl<Success, Error> RetryOutcome<Success, Error> {
    #[must_use]
    pub const fn attempts(
        &self,
    ) -> crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize {
        self.attempts
    }

    pub fn into_result(self) -> Result<Success, Error> {
        self.result
    }
}
