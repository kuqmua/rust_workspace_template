#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("normalized HTTP path is too long")]
pub struct HttpNormalizedPathError;
