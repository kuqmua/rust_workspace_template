#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum LocationCoordinateTryFromU32Error {
    #[error("{self:?}")]
    OutOfRange,
}
