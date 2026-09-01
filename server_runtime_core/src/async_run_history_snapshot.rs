#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
#[constructor(pub(crate))]
pub struct AsyncRunHistorySnapshot<RunReport> {
    #[getters(skip)]
    latest_report: Option<RunReport>,
    #[getters(copy)]
    report_count: super::std_async_run_history_report_count::StdAsyncRunHistoryReportCount,
}
impl<RunReport> AsyncRunHistorySnapshot<RunReport> {
    #[must_use]
    pub const fn latest_report(&self) -> Option<&RunReport> {
        self.latest_report.as_ref()
    }
}
