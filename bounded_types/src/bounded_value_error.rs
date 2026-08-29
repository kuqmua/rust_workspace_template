#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum BoundedValueError {
    #[error("bounded value length {actual} exceeds maximum {max}")]
    AboveMax {
        actual: crate::bounded_len::BoundedLen,
        max: crate::bounded_len::BoundedLen,
    },
    #[error("bounded value length {actual} is below minimum {min}")]
    BelowMin {
        actual: crate::bounded_len::BoundedLen,
        min: crate::bounded_len::BoundedLen,
    },
    #[error("bounded value minimum {min} exceeds maximum {max}")]
    InvalidBounds {
        min: crate::bounded_len::BoundedLen,
        max: crate::bounded_len::BoundedLen,
    },
}
