#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum LeaseTextError {
    #[error("lease text contains a NUL character")]
    ContainsNul,
    #[error("lease text must not be empty")]
    Empty,
    #[error("lease text exceeds its maximum length")]
    TooLong,
}
