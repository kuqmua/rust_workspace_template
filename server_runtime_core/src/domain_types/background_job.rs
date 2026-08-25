#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct BackgroundJob<Report> {
    history: crate::domain_types::AsyncRunHistory<Report>,
    retry_policy: crate::domain_types::RetryPolicy,
}

impl<Report: Clone + Send + Sync + 'static> BackgroundJob<Report> {
    #[must_use]
    pub const fn new(
        history: crate::domain_types::AsyncRunHistory<Report>,
        retry_policy: crate::domain_types::RetryPolicy,
    ) -> Self {
        Self {
            history,
            retry_policy,
        }
    }

    pub async fn run_once<Run, RunFuture, Success, Error, IsRetryable, MapReport>(
        &self,
        run: Run,
        is_retryable: IsRetryable,
        map_report: MapReport,
    ) -> Report
    where
        Run: FnMut() -> RunFuture,
        RunFuture: Future<Output = Result<Success, Error>>,
        IsRetryable: Fn(&Error) -> bool,
        MapReport: FnOnce(crate::domain_types::RetryOutcome<Success, Error>) -> Report,
    {
        let outcome =
            crate::domain_types::run_with_retries(self.retry_policy, run, is_retryable).await;
        let report = map_report(outcome);
        self.history.push(report.clone()).await;
        report
    }

    pub async fn snapshot(&self) -> crate::domain_types::AsyncRunHistorySnapshot<Report> {
        self.history.snapshot().await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn run_records_retry_outcome_in_bounded_history() {
        let history = crate::domain_types::AsyncRunHistory::new(
            crate::domain_types::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(
                constants_usize::ONE,
            )
            .expect("5dc81fa2 run_records_retry_outcome_in_bounded_history invariant must hold"),
        );
        let job = super::BackgroundJob::new(
            history,
            crate::domain_types::RetryPolicy::new(
                crate::domain_types::RetryAttemptsNonZeroUsize::try_from(2usize).expect(
                    "4792b3e0 run_records_retry_outcome_in_bounded_history invariant must hold",
                ),
                None,
            ),
        );
        let mut calls = constants_usize::ZERO;
        let report = job
            .run_once(
                || {
                    calls = calls.saturating_add(constants_usize::ONE);
                    std::future::ready(if calls == constants_usize::ONE {
                        Err(())
                    } else {
                        Ok(7usize)
                    })
                },
                |()| true,
                |outcome| (outcome.attempts().get(), outcome.into_result()),
            )
            .await;
        assert_eq!(report, (2usize, Ok(7usize)));
        assert_eq!(job.snapshot().await.latest_report(), Some(&report));
    }
}
