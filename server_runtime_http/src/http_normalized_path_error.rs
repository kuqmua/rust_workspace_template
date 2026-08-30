#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpNormalizedPathError {
    #[error("normalized HTTP path is too long")]
    TooLarge,
}
