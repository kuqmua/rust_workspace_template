#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum BoundedTextPolicyError {
    #[error("text contains a NUL character")]
    ContainsNul,
    #[error("text must not be empty")]
    Empty,
    #[error("text exceeds its maximum byte length")]
    TooLong,
}
