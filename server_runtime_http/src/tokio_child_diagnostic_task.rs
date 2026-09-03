#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
)]
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
}
