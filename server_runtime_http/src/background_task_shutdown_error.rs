#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BackgroundTaskShutdownError {
    #[error("background task failed: {0}")]
    Join(#[source] crate::tokio_task_join_error::TokioTaskJoinError),
    #[error("{}", constants_str::catalog::BACKGROUND_TASK_SHUTDOWN_TIMED_OUT)]
    Timeout,
}
