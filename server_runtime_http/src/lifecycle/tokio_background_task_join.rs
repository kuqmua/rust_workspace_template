#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioBackgroundTaskJoin(
    pub(super) tokio::task::JoinHandle<super::BackgroundTaskOutcome>,
);
