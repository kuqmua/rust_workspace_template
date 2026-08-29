#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum CursorCodecBuildError {
    #[error("{message}", message = constants_str::catalog::CURSOR_MAXIMUM_LENGTH_MUST_BE_GREATER_THAN_ZERO)]
    ZeroMaximumLength,
}
