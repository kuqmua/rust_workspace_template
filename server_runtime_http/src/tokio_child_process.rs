#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub struct TokioChildProcess(tokio::process::Child);

impl TokioChildProcess {
    pub(crate) fn into_inner(self) -> tokio::process::Child {
        self.0
    }
}
