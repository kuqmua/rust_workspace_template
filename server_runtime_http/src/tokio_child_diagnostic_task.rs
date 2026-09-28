#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
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
    pub(super) async fn join(
        &mut self,
    ) -> Result<
        crate::child_diagnostic::ChildDiagnostic,
        crate::child_process_error::ChildProcessError,
    > {
        (&mut self.0)
            .await
            .map_err(crate::tokio_child_process_join_error::TokioChildProcessJoinError::from)
            .map_err(crate::child_process_error::ChildProcessError::Join)?
    }

    pub(super) fn abort(&self) {
        self.0.abort();
    }
}
