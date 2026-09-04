#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
pub struct BackgroundJob<Report> {
    history: crate::async_run_history::AsyncRunHistory<Report>,
    retry_policy: crate::retry_policy::RetryPolicy,
}

impl<Report: Clone + Send + Sync + 'static> BackgroundJob<Report> {
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
        MapReport: FnOnce(crate::retry_outcome::RetryOutcome<Success, Error>) -> Report,
    {
        let outcome =
            crate::run_with_retries::run_with_retries(self.retry_policy, run, is_retryable).await;
        let report = map_report(outcome);
        self.history.push(report.clone()).await;
        report
    }

    pub async fn snapshot(
        &self,
    ) -> crate::async_run_history_snapshot::AsyncRunHistorySnapshot<Report> {
        self.history.snapshot().await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_run_records_retry_outcome_in_bounded_history() {
        let history = crate::async_run_history::AsyncRunHistory::new(
            crate::async_run_history_maximum_len_non_zero_usize::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(constants_usize::ONE).expect(constants_str::DIAGNOSTIC_5DC81FA2),
        );
        let job = super::BackgroundJob::new(
            history,
            crate::retry_policy::RetryPolicy::new(
                crate::retry_attempts_non_zero_usize::RetryAttemptsNonZeroUsize::try_from(2usize)
                    .expect(constants_str::DIAGNOSTIC_4792B3E0),
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
