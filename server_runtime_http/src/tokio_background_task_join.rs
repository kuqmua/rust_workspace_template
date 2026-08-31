#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(super) struct TokioBackgroundTaskJoin(
    tokio::task::JoinHandle<crate::background_task_outcome::BackgroundTaskOutcome>,
);

impl TokioBackgroundTaskJoin {
    pub(super) fn abort(&self) {
        self.0.abort();
    }
}
