#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum SingleFlightKeyError {
    #[error("single-flight key contains a NUL character")]
    ContainsNul,
    #[error("single-flight key must not be empty")]
    Empty,
    #[error("single-flight key exceeds its maximum length")]
    TooLong,
}
