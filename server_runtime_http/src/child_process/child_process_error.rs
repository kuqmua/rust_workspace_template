#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ChildProcessError {
    #[error("child process diagnostic read failed")]
    DiagnosticIo(super::ChildProcessIoError),
    #[error("child process diagnostic buffer range is invalid")]
    DiagnosticRange,
    #[error("child process operation failed")]
    Io(super::ChildProcessIoError),
    #[error("child process diagnostic task failed")]
    Join(super::TokioChildProcessJoinError),
    #[error("child process is missing")]
    MissingChild,
    #[error("child process did not terminate before the timeout")]
    Timeout,
}
