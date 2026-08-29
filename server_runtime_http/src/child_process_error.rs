#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ChildProcessError {
    #[error("child process diagnostic read failed")]
    DiagnosticIo(crate::child_process_io_error::ChildProcessIoError),
    #[error("child process diagnostic buffer range is invalid")]
    DiagnosticRange,
    #[error("child process operation failed")]
    Io(crate::child_process_io_error::ChildProcessIoError),
    #[error("child process diagnostic task failed")]
    Join(crate::tokio_child_process_join_error::TokioChildProcessJoinError),
    #[error("child process is missing")]
    MissingChild,
    #[error("child process did not terminate before the timeout")]
    Timeout,
}
