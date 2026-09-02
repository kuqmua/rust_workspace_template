#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    thiserror::Error,
)]
pub enum FiniteF64Error {
    #[error("floating-point value must be finite")]
    NotFinite,
}
