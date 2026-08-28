#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum BoundedValueError {
    #[error("bounded value length {actual} exceeds maximum {max}")]
    AboveMax {
        actual: crate::BoundedLen,
        max: crate::BoundedLen,
    },
    #[error("bounded value length {actual} is below minimum {min}")]
    BelowMin {
        actual: crate::BoundedLen,
        min: crate::BoundedLen,
    },
    #[error("bounded value minimum {min} exceeds maximum {max}")]
    InvalidBounds {
        min: crate::BoundedLen,
        max: crate::BoundedLen,
    },
}
