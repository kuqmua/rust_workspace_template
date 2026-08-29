#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ServeWithGracefulShutdownError {
    #[error("server failed: {0}")]
    Serve(#[source] crate::serve_io_error::ServeIoError),
    #[error("{}", constants_str::catalog::SERVER_GRACEFUL_SHUTDOWN_TIMED_OUT)]
    ShutdownTimeout,
}
