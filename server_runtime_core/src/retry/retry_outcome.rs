#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
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
