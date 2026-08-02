#[derive(optml::Optml, Clone, Debug)]
pub struct BackgroundJob<Report> {
    history: crate::AsyncRunHistory<Report>,
    retry_policy: crate::RetryPolicy,
}

impl<Report: Clone + Send + Sync + 'static> BackgroundJob<Report> {
    #[must_use]
    pub const fn new(
        history: crate::AsyncRunHistory<Report>,
        retry_policy: crate::RetryPolicy,
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
        MapReport: FnOnce(crate::RetryOutcome<Success, Error>) -> Report,
    {
        let outcome = crate::run_with_retries(self.retry_policy, run, is_retryable).await;
        let report = map_report(outcome);
        self.history.push(report.clone()).await;
        report
    }

    pub async fn snapshot(&self) -> crate::AsyncRunHistorySnapshot<Report> {
        self.history.snapshot().await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn run_records_retry_outcome_in_bounded_history() {
        let history = crate::AsyncRunHistory::new(
            crate::StdAsyncRunHistoryMaximumLen::try_from(1usize).expect("5dc81fa2"),
        );
        let job = super::BackgroundJob::new(
            history,
            crate::RetryPolicy::new(
                crate::StdRetryAttempts::try_from(2usize).expect("4792b3e0"),
                None,
            ),
        );
        let mut calls = 0usize;
        let report = job
            .run_once(
                || {
                    calls = calls.saturating_add(1usize);
                    std::future::ready(if calls == 1usize { Err(()) } else { Ok(7usize) })
                },
                |()| true,
                |outcome| (outcome.attempts().get(), outcome.into_result()),
            )
            .await;
        assert_eq!(report, (2usize, Ok(7usize)));
        assert_eq!(job.snapshot().await.latest_report(), Some(&report));
    }
}
