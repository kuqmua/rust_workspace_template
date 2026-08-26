#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the history owner module accesses this private shared storage wrapper"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::CloneInner,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct SharedRunReportsArc<RunReport>(
    std::sync::Arc<
        tokio::sync::RwLock<super::run_reports_vec_deque::RunReportsVecDeque<RunReport>>,
    >,
);
