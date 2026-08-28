#[path = "retry_attempts_non_zero_usize.rs"]
mod retry_attempts_non_zero_usize;
#[path = "retry_delay_duration.rs"]
mod retry_delay_duration;
#[path = "retry_outcome.rs"]
mod retry_outcome;
#[path = "retry_policy.rs"]
mod retry_policy;
#[path = "run_with_retries.rs"]
mod run_with_retries;
#[path = "std_retry_attempts_error.rs"]
mod std_retry_attempts_error;

pub use retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize;
pub use retry_delay_duration::RetryDelayDuration;
pub use retry_outcome::RetryOutcome;
pub use retry_policy::RetryPolicy;
pub use run_with_retries::run_with_retries;
pub use std_retry_attempts_error::StdRetryAttemptsError;

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
