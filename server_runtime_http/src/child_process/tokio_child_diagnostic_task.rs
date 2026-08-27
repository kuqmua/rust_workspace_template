#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioChildDiagnosticTask(
    pub(super) tokio::task::JoinHandle<Result<super::ChildDiagnostic, super::ChildProcessError>>,
);
