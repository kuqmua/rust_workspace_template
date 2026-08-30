#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminIdTryFromI64Error {
    #[error("{self:?}")]
    Invalid,
}
