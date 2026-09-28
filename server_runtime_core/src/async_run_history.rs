#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_clone_fields::CloneFields,
)]
pub struct AsyncRunHistory<RunReport> {
    maximum_len:
        super::async_run_history_maximum_len_non_zero_usize::AsyncRunHistoryMaximumLenNonZeroUsize,
    reports: super::shared_run_reports_arc::SharedRunReportsArc<RunReport>,
}
impl<RunReport: Send + Sync> AsyncRunHistory<RunReport> {
    #[must_use]
    pub fn new(
        async_run_history_maximum_len_non_zero_usize: super::async_run_history_maximum_len_non_zero_usize::AsyncRunHistoryMaximumLenNonZeroUsize,
    ) -> Self {
        let reports = super::run_reports_vec_deque::RunReportsVecDeque::from(
            std::collections::VecDeque::with_capacity(
                async_run_history_maximum_len_non_zero_usize
                    .get()
                    .min(constants_usize::VALUE_4_096),
            ),
        );
        Self {
            maximum_len: async_run_history_maximum_len_non_zero_usize,
            reports: super::shared_run_reports_arc::SharedRunReportsArc::from(
                std::sync::Arc::from(tokio::sync::RwLock::new(reports)),
            ),
        }
    }
    pub async fn push(&self, run_report: RunReport) {
        let mut reports = self.reports.write().await;
        if reports.len() == self.maximum_len.get() {
            let _removed = reports.pop_front();
        }
        reports.push_back(run_report);
    }
}
impl<RunReport: Clone + Send + Sync> AsyncRunHistory<RunReport> {
    pub async fn snapshot(
        &self,
    ) -> super::async_run_history_snapshot::AsyncRunHistorySnapshot<RunReport> {
        let reports = self.reports.read().await;
        super::async_run_history_snapshot::AsyncRunHistorySnapshot::new(
            reports.back().cloned(),
            super::std_async_run_history_report_count::StdAsyncRunHistoryReportCount::from(
                reports.len(),
            ),
        )
    }
}
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_history_does_not_allocate_maximum_upfront() {
        let maximum = crate::async_run_history_maximum_len_non_zero_usize::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(
            usize::from(std::num::NonZeroUsize::MAX),
        ).expect(constants_str::DIAGNOSTIC_DCDE8137);
        let history = super::AsyncRunHistory::new(maximum);
        history.push(1u8).await;
        let snapshot = history.snapshot().await;
        assert_eq!(usize::from(snapshot.report_count()), constants_usize::ONE);
        assert_eq!(snapshot.latest_report(), Some(&1u8));
    }

    #[test]
    fn test_history_clone_does_not_require_report_clone() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
        struct NotClone;
        let maximum = crate::async_run_history_maximum_len_non_zero_usize::AsyncRunHistoryMaximumLenNonZeroUsize::try_from(constants_usize::ONE)
            .expect(constants_str::DIAGNOSTIC_91F5D3A8);
        let history = super::AsyncRunHistory::<NotClone>::new(maximum);
        let cloned = history.clone();
        assert_eq!(history.maximum_len, cloned.maximum_len, "f1c763a4");
    }
}
