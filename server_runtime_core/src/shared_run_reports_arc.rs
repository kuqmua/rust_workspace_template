#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::CloneInner,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SharedRunReportsArc<RunReport>(
    std::sync::Arc<
        tokio::sync::RwLock<super::run_reports_vec_deque::RunReportsVecDeque<RunReport>>,
    >,
);
