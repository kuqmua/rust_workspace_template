#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("password maximum length must not be less than minimum length")]
pub struct PasswordLengthRangeError;
