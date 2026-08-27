#[derive(
    Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum MaximumSizeOfHttpBodyInBytesTryFromUsizeError {
    #[error("maximum size of http body in bytes must be greater than zero")]
    IsZero,
}
