#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_retryable_failure_is_retried_until_success() {
        let mut calls = constants_usize::ZERO;
        let outcome = crate::run_with_retries::run_with_retries(
            crate::retry_policy::RetryPolicy::new(
                crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::try_from(3usize)
                    .expect(constants_str::DIAGNOSTIC_E7BC9A41),
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
                    .expect(constants_str::DIAGNOSTIC_61B6AED5),
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
    #[tokio::test]
    async fn test_retry_immediate_success_skips_error_classifier() {
        let mut calls = constants_usize::ZERO;
        let classifications = std::cell::Cell::new(constants_usize::ZERO);
        let outcome = crate::run_with_retries::run_with_retries(
            crate::retry_policy::RetryPolicy::new(
                crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::from(
                    std::num::NonZeroUsize::MIN.saturating_add(constants_usize::TWO),
                ),
                None,
            ),
            || {
                calls += constants_usize::ONE;
                std::future::ready(Ok::<usize, usize>(7usize))
            },
            |_| {
                classifications.set(classifications.get() + constants_usize::ONE);
                true
            },
        )
        .await;
        assert_eq!(calls, constants_usize::ONE);
        assert_eq!(classifications.get(), constants_usize::ZERO);
        assert_eq!(outcome.attempts().get(), constants_usize::ONE);
        assert_eq!(outcome.into_result(), Ok(7usize));
    }

    #[tokio::test]
    async fn test_retry_exhaustion_preserves_final_error_and_skips_final_classification() {
        let mut calls = constants_usize::ZERO;
        let classifications = std::cell::Cell::new(constants_usize::ZERO);
        let outcome = crate::run_with_retries::run_with_retries(
            crate::retry_policy::RetryPolicy::new(
                crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::from(
                    std::num::NonZeroUsize::MIN.saturating_add(constants_usize::TWO),
                ),
                None,
            ),
            || {
                calls += constants_usize::ONE;
                std::future::ready(Err::<(), usize>(calls))
            },
            |error| {
                classifications.set(classifications.get() + constants_usize::ONE);
                assert_eq!(*error, classifications.get());
                true
            },
        )
        .await;
        assert_eq!(calls, constants_usize::THREE);
        assert_eq!(classifications.get(), constants_usize::TWO);
        assert_eq!(outcome.attempts().get(), constants_usize::THREE);
        assert_eq!(outcome.into_result(), Err(constants_usize::THREE));
    }
    #[test]
    fn test_retry_attempt_count_rejects_zero_and_preserves_positive_values() {
        assert_eq!(
            crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::try_from(
                constants_usize::ZERO
            ),
            Err(crate::std_retry_attempts_error::StdRetryAttemptsError::Zero)
        );
        [constants_usize::ONE, constants_usize::THREE]
            .into_iter()
            .fold((), |(), attempts| {
                assert!(
                    crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::try_from(
                        attempts
                    )
                    .is_ok_and(|value| value.get() == attempts)
                );
            });
    }

    #[tokio::test]
    async fn test_single_attempt_failure_skips_classifier_and_preserves_error() {
        let classifications = std::cell::Cell::new(constants_usize::ZERO);
        let outcome = crate::run_with_retries::run_with_retries(
            crate::retry_policy::RetryPolicy::new(
                crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::from(
                    std::num::NonZeroUsize::MIN,
                ),
                None,
            ),
            || std::future::ready(Err::<(), usize>(7usize)),
            |_| {
                classifications.set(classifications.get() + constants_usize::ONE);
                true
            },
        )
        .await;
        assert_eq!(classifications.get(), constants_usize::ZERO);
        assert_eq!(outcome.attempts().get(), constants_usize::ONE);
        assert_eq!(outcome.into_result(), Err(7usize));
    }
}
