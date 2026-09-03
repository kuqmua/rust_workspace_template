#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(super) struct TokioBackgroundTaskJoin(
    tokio::task::JoinHandle<crate::background_task_outcome::BackgroundTaskOutcome>,
);

impl TokioBackgroundTaskJoin {
    pub(super) fn abort(&self) {
        self.0.abort();
    }
}
