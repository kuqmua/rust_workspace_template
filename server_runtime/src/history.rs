#[derive(Debug)]
struct StdVecDequeRunReports<RunReport>(std::collections::VecDeque<RunReport>);
#[derive(Debug)]
struct TokioRwLockRunReports<RunReport>(tokio::sync::RwLock<StdVecDequeRunReports<RunReport>>);
#[derive(Debug)]
struct StdArcSharedRunReports<RunReport>(std::sync::Arc<TokioRwLockRunReports<RunReport>>);
impl<RunReport> Clone for StdArcSharedRunReports<RunReport> {
    fn clone(&self) -> Self {
        Self(std::sync::Arc::clone(&self.0))
    }
}
#[derive(Debug)]
pub struct AsyncRunHistory<RunReport> {
    maximum_len: StdAsyncRunHistoryMaximumLen,
    reports: StdArcSharedRunReports<RunReport>,
}
impl<RunReport> Clone for AsyncRunHistory<RunReport> {
    fn clone(&self) -> Self {
        Self {
            maximum_len: self.maximum_len,
            reports: self.reports.clone(),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAsyncRunHistoryMaximumLen(std::num::NonZeroUsize);
impl TryFrom<usize> for StdAsyncRunHistoryMaximumLen {
    type Error = StdAsyncRunHistoryMaximumLenTryFromUsizeError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or(StdAsyncRunHistoryMaximumLenTryFromUsizeError)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAsyncRunHistoryMaximumLenTryFromUsizeError;
impl std::fmt::Display for StdAsyncRunHistoryMaximumLenTryFromUsizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::RUN_HISTORY_MAXIMUM_LENGTH_MUST_BE_GREATER_THAN_ZERO)
    }
}
impl std::error::Error for StdAsyncRunHistoryMaximumLenTryFromUsizeError {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAsyncRunHistoryReportCount(usize);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncRunHistorySnapshot<RunReport> {
    latest_report: Option<RunReport>,
    report_count: StdAsyncRunHistoryReportCount,
}
impl<RunReport> AsyncRunHistorySnapshot<RunReport> {
    #[must_use]
    pub const fn latest_report(&self) -> Option<&RunReport> {
        self.latest_report.as_ref()
    }
    #[must_use]
    pub const fn report_count(&self) -> StdAsyncRunHistoryReportCount {
        self.report_count
    }
}
impl From<StdAsyncRunHistoryReportCount> for usize {
    fn from(value: StdAsyncRunHistoryReportCount) -> Self {
        value.0
    }
}
impl<RunReport: Send + Sync> AsyncRunHistory<RunReport> {
    #[must_use]
    pub fn new(maximum_len: StdAsyncRunHistoryMaximumLen) -> Self {
        let reports = StdVecDequeRunReports(std::collections::VecDeque::with_capacity(
            maximum_len.0.get(),
        ));
        Self {
            maximum_len,
            reports: StdArcSharedRunReports(std::sync::Arc::from(TokioRwLockRunReports(
                tokio::sync::RwLock::new(reports),
            ))),
        }
    }
    pub async fn push(&self, report: RunReport) {
        let mut reports = self.reports.0.0.write().await;
        if reports.0.len() == self.maximum_len.0.get() {
            let _removed = reports.0.pop_front();
        }
        reports.0.push_back(report);
    }
}
impl<RunReport: Clone + Send + Sync> AsyncRunHistory<RunReport> {
    pub async fn snapshot(&self) -> AsyncRunHistorySnapshot<RunReport> {
        let reports = self.reports.0.0.read().await;
        AsyncRunHistorySnapshot {
            latest_report: reports.0.back().cloned(),
            report_count: StdAsyncRunHistoryReportCount(reports.0.len()),
        }
    }
}
