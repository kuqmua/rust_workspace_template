#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub struct RetryOutcome<Success, Error> {
    pub(super) attempts: super::RetryAttemptsNonZeroUsize,
    pub(super) result: Result<Success, Error>,
}

impl<Success, Error> RetryOutcome<Success, Error> {
    #[must_use]
    pub const fn attempts(&self) -> super::RetryAttemptsNonZeroUsize {
        self.attempts
    }

    pub fn into_result(self) -> Result<Success, Error> {
        self.result
    }
}
