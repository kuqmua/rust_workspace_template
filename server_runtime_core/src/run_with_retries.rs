pub async fn run_with_retries<Run, RunFuture, Success, Error, IsRetryable>(
    policy: super::RetryPolicy,
    mut run: Run,
    is_retryable: IsRetryable,
) -> super::RetryOutcome<Success, Error>
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
            return super::RetryOutcome {
                attempts: super::RetryAttemptsNonZeroUsize::from(
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
