#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub(super) enum GeneratePgTypesLengthError {
    #[error("{self:?}")]
    TooLarge,
}
