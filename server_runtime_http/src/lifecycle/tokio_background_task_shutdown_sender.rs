#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioBackgroundTaskShutdownSender(pub(super) tokio::sync::oneshot::Sender<()>);
