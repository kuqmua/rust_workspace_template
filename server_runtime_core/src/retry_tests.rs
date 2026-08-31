#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_retryable_failure_is_retried_until_success() {
        let mut calls = constants_usize::ZERO;
        let outcome = crate::run_with_retries::run_with_retries(
            crate::retry_policy::RetryPolicy::new(
                crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::try_from(3usize)
                    .expect(
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
    async fn test_terminal_failure_is_not_retried() {
        let mut calls = constants_usize::ZERO;
        let outcome = crate::run_with_retries::run_with_retries(
            crate::retry_policy::RetryPolicy::new(
                crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::try_from(3usize)
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
