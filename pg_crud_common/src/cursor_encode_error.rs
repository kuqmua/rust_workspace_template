#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum CursorEncodeError {
    #[error("{message}", message = constants_str::catalog::CURSOR_SIGNING_KEY_IS_INVALID)]
    InvalidSigningKey,
    #[error("{message}", message = constants_str::catalog::CURSOR_EXCEEDS_MAXIMUM_LENGTH)]
    MaximumLengthExceeded,
}
