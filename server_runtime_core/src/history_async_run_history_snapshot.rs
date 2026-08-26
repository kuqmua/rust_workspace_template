#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the history owner module constructs snapshots while fields remain private outside the facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct AsyncRunHistorySnapshot<RunReport> {
    pub(super) latest_report: Option<RunReport>,
    pub(super) report_count:
        super::std_async_run_history_report_count::StdAsyncRunHistoryReportCount,
}
impl<RunReport> AsyncRunHistorySnapshot<RunReport> {
    #[must_use]
    pub const fn latest_report(&self) -> Option<&RunReport> {
        self.latest_report.as_ref()
    }
    #[must_use]
    pub const fn report_count(
        &self,
    ) -> super::std_async_run_history_report_count::StdAsyncRunHistoryReportCount {
        self.report_count
    }
}
