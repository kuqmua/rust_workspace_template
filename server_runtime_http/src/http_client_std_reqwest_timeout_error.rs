#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("HTTP client timeout must be greater than zero")]
pub struct StdReqwestTimeoutError;
