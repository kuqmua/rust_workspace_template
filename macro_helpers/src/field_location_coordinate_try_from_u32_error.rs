#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum FieldLocationCoordinateTryFromU32Error {
    #[error("{self:?}")]
    OutOfRange,
}
