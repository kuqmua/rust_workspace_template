#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct TokioAbortTask(pub(super) tokio::task::JoinHandle<()>);
