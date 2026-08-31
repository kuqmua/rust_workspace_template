#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioChildDiagnosticTask(
    tokio::task::JoinHandle<
        Result<
            crate::child_diagnostic::ChildDiagnostic,
            crate::child_process_error::ChildProcessError,
        >,
    >,
);

impl TokioChildDiagnosticTask {
    pub(super) fn abort(&self) {
        self.0.abort();
    }

    pub(super) fn into_inner(
        self,
    ) -> tokio::task::JoinHandle<
        Result<
            crate::child_diagnostic::ChildDiagnostic,
            crate::child_process_error::ChildProcessError,
        >,
    > {
        self.0
    }
}
