#[derive(
    Clone, Copy, Debug, Eq, PartialEq, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum BoundedStringError {
    #[error("string length {actual_length} is above maximum {maximum_length}")]
    AboveMaximum {
        actual_length: crate::bounded_len::BoundedLen,
        maximum_length: crate::bounded_len::BoundedLen,
    },
    #[error("string length {actual_length} is below minimum {minimum_length}")]
    BelowMinimum {
        actual_length: crate::bounded_len::BoundedLen,
        minimum_length: crate::bounded_len::BoundedLen,
    },
}
