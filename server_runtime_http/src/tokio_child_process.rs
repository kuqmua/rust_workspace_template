#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct TokioChildProcess(tokio::process::Child);

impl TokioChildProcess {
    pub(crate) fn into_inner(self) -> tokio::process::Child {
        self.0
    }
}
