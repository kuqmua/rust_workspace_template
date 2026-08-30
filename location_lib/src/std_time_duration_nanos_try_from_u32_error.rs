#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum StdTimeDurationNanosTryFromU32Error {
    #[error("{self:?}")]
    OutOfRange,
}
