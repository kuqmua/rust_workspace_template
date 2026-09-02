#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    thiserror::Error,
)]
pub enum UnitIntervalF64Error {
    #[error("floating-point value must be finite")]
    NotFinite,
    #[error("floating-point value must be within the inclusive unit interval")]
    OutOfRange,
}
