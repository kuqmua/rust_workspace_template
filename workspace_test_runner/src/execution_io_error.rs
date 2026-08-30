#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(super) enum ExecutionIoError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
