#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, thiserror::Error,
)]
pub enum PositiveFiniteF64Error {
    #[error("floating-point value must be finite")]
    NotFinite,
    #[error("floating-point value must be greater than zero")]
    NotPositive,
}
