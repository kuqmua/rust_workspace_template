#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_clone_inner::CloneInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct SharedRunReportsArc<RunReport>(
    std::sync::Arc<
        tokio::sync::RwLock<super::run_reports_vec_deque::RunReportsVecDeque<RunReport>>,
    >,
);
