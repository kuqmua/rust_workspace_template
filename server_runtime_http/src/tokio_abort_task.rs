#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub struct TokioAbortTask(tokio::task::JoinHandle<()>);

impl TokioAbortTask {
    pub(crate) fn into_inner(self) -> tokio::task::JoinHandle<()> {
        self.0
    }
}
