#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum BoundedStringStorageError {
    #[error("string length {actual_length} is above maximum {maximum_length}")]
    AboveMaximum {
        actual_length: usize,
        maximum_length: usize,
    },
    #[error("string length {actual_length} is below minimum {minimum_length}")]
    BelowMinimum {
        actual_length: usize,
        minimum_length: usize,
    },
}
