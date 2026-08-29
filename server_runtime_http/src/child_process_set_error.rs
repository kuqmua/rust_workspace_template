#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ChildProcessSetError {
    #[error("child process set is full")]
    Full,
    #[error("child process identifier overflowed")]
    IdOverflow,
    #[error("child process shutdown failed")]
    Process(#[source] crate::child_process_error::ChildProcessError),
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for ChildProcessSetError {
    fn from(_value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        Self::Full
    }
}
