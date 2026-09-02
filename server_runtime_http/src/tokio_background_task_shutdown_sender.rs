#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(super) struct TokioBackgroundTaskShutdownSender(tokio::sync::oneshot::Sender<()>);
