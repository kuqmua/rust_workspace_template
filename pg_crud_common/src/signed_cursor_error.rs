#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = constants_str::catalog::SIGNED_CURSOR_MUST_NOT_BE_EMPTY)]
pub struct SignedCursorError;
