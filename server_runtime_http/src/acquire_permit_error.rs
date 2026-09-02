#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AcquirePermitError {
    #[error("concurrency limiter is closed: {0}")]
    Closed(#[source] crate::tokio_acquire_error::TokioAcquireError),
    #[error("concurrency limit reached; retry after {} seconds", .0.get())]
    Timeout(crate::retry_after_secs::RetryAfterSecs),
}
