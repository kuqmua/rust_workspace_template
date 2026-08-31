#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(super) struct TokioBackgroundTaskShutdownSender(tokio::sync::oneshot::Sender<()>);
