#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct TokioAbortTask(tokio::task::JoinHandle<()>);

impl TokioAbortTask {
    pub(crate) fn into_inner(self) -> tokio::task::JoinHandle<()> {
        self.0
    }
}
