#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum CursorPayloadError {
    #[error("{message}", message = constants_str::catalog::CURSOR_PAYLOAD_MUST_NOT_BE_EMPTY)]
    Empty,
}
