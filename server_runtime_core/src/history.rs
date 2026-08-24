#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct StdVecDequeRunReports<RunReport>(std::collections::VecDeque<RunReport>);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::CloneInner, newtype::FromInner,
)]
struct StdArcSharedRunReports<RunReport>(
    std::sync::Arc<tokio::sync::RwLock<StdVecDequeRunReports<RunReport>>>,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::CloneFields)]
pub struct AsyncRunHistory<RunReport> {
    maximum_len: StdAsyncRunHistoryMaximumLen,
    reports: StdArcSharedRunReports<RunReport>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAsyncRunHistoryMaximumLen(std::num::NonZeroUsize);
impl TryFrom<usize> for StdAsyncRunHistoryMaximumLen {
    type Error = StdAsyncRunHistoryMaximumLenTryFromUsizeError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or(StdAsyncRunHistoryMaximumLenTryFromUsizeError)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error(
    "{}",
    str_constants::RUN_HISTORY_MAXIMUM_LENGTH_MUST_BE_GREATER_THAN_ZERO
)]
pub struct StdAsyncRunHistoryMaximumLenTryFromUsizeError;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct StdAsyncRunHistoryReportCount(usize);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
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
impl<RunReport: Send + Sync> AsyncRunHistory<RunReport> {
    #[must_use]
    pub fn new(maximum_len: StdAsyncRunHistoryMaximumLen) -> Self {
        let reports = StdVecDequeRunReports::from(std::collections::VecDeque::with_capacity(
            maximum_len.0.get(),
        ));
        Self {
            maximum_len,
            reports: StdArcSharedRunReports::from(std::sync::Arc::from(tokio::sync::RwLock::new(
                reports,
            ))),
        }
    }
    pub async fn push(&self, report: RunReport) {
        let mut reports = self.reports.0.write().await;
        if reports.0.len() == self.maximum_len.0.get() {
            let _removed = reports.0.pop_front();
        }
        reports.0.push_back(report);
    }
}
impl<RunReport: Clone + Send + Sync> AsyncRunHistory<RunReport> {
    pub async fn snapshot(&self) -> AsyncRunHistorySnapshot<RunReport> {
        let reports = self.reports.0.read().await;
        AsyncRunHistorySnapshot {
            latest_report: reports.0.back().cloned(),
            report_count: StdAsyncRunHistoryReportCount::from(reports.0.len()),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn history_clone_does_not_require_report_clone() {
        #[derive(optimal_memory_layout::OptimalMemoryLayout)]
        struct NotClone;
        let maximum = super::StdAsyncRunHistoryMaximumLen::try_from(1usize)
            .expect("91f5d3a8 history_clone_does_not_require_report_clone invariant must hold");
        let history = super::AsyncRunHistory::<NotClone>::new(maximum);
        let cloned = history.clone();
        assert_eq!(history.maximum_len, cloned.maximum_len, "f1c763a4");
    }
}
