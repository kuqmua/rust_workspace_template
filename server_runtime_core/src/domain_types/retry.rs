#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct RetryAttemptsNonZeroUsize(std::num::NonZeroUsize);

impl RetryAttemptsNonZeroUsize {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

impl TryFrom<usize> for RetryAttemptsNonZeroUsize {
    type Error = StdRetryAttemptsError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or(StdRetryAttemptsError)
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("retry attempts must be greater than zero")]
pub struct StdRetryAttemptsError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct RetryDelayDuration(std::time::Duration);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    attempts: RetryAttemptsNonZeroUsize,
    delay: Option<RetryDelayDuration>,
}

impl RetryPolicy {
    #[must_use]
    pub const fn attempts(self) -> RetryAttemptsNonZeroUsize {
        self.attempts
    }

    #[must_use]
    pub const fn delay(self) -> Option<RetryDelayDuration> {
        self.delay
    }

    #[must_use]
    pub const fn new(
        attempts: RetryAttemptsNonZeroUsize,
        delay: Option<RetryDelayDuration>,
    ) -> Self {
        Self { attempts, delay }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub struct RetryOutcome<Success, Error> {
    attempts: RetryAttemptsNonZeroUsize,
    result: Result<Success, Error>,
}

impl<Success, Error> RetryOutcome<Success, Error> {
    #[must_use]
    pub const fn attempts(&self) -> RetryAttemptsNonZeroUsize {
        self.attempts
    }

    pub fn into_result(self) -> Result<Success, Error> {
        self.result
    }
}

pub async fn run_with_retries<Run, RunFuture, Success, Error, IsRetryable>(
    policy: RetryPolicy,
    mut run: Run,
    is_retryable: IsRetryable,
) -> RetryOutcome<Success, Error>
where
    Run: FnMut() -> RunFuture,
    RunFuture: Future<Output = Result<Success, Error>>,
    IsRetryable: Fn(&Error) -> bool,
{
    let maximum_attempts = policy.attempts().get();
    let mut attempt = constants_usize::ONE;
    loop {
        let result = run().await;
        let should_retry = result
            .as_ref()
            .is_err_and(|error| attempt < maximum_attempts && is_retryable(error));
        if !should_retry {
            return RetryOutcome {
                attempts: RetryAttemptsNonZeroUsize::from(
                    std::num::NonZeroUsize::new(attempt).unwrap_or(std::num::NonZeroUsize::MIN),
                ),
                result,
            };
        }
        if let Some(delay) = policy.delay() {
            tokio::time::sleep(delay.0).await;
        }
        attempt = attempt.saturating_add(constants_usize::ONE);
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn retryable_failure_is_retried_until_success() {
        let mut calls = constants_usize::ZERO;
        let outcome = super::run_with_retries(
            super::RetryPolicy::new(
                super::RetryAttemptsNonZeroUsize::try_from(3usize).expect(
                    "e7bc9a41 retryable_failure_is_retried_until_success invariant must hold",
                ),
                None,
            ),
            || {
                calls = calls.saturating_add(constants_usize::ONE);
                std::future::ready(if calls < 3usize { Err(()) } else { Ok(7usize) })
            },
            |()| true,
        )
        .await;
        assert_eq!(outcome.attempts().get(), 3usize);
        assert_eq!(outcome.into_result(), Ok(7usize));
    }

    #[tokio::test]
    async fn terminal_failure_is_not_retried() {
        let mut calls = constants_usize::ZERO;
        let outcome = super::run_with_retries(
            super::RetryPolicy::new(
                super::RetryAttemptsNonZeroUsize::try_from(3usize)
                    .expect("61b6aed5 terminal_failure_is_not_retried invariant must hold"),
                None,
            ),
            || {
                calls = calls.saturating_add(constants_usize::ONE);
                std::future::ready(Err::<(), usize>(calls))
            },
            |_| false,
        )
        .await;
        assert_eq!(outcome.attempts().get(), constants_usize::ONE);
        assert_eq!(outcome.into_result(), Err(constants_usize::ONE));
    }
}
