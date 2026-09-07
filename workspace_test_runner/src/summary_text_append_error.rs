#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum SummaryTextAppendError {
    #[error("{}", constants_str::RUNNER_SUMMARY_LIMIT_MESSAGE)]
    CapacityExceeded,
}
