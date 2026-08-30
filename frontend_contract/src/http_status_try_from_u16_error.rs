#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpStatusTryFromU16Error {
    #[error("{self:?}")]
    OutOfRange,
}
