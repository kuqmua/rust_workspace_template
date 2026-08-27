#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum BoundedValueError {
    #[error("bounded value length {actual} exceeds maximum {max}")]
    AboveMax {
        actual: crate::domain_types::BoundedLen,
        max: crate::domain_types::BoundedLen,
    },
    #[error("bounded value length {actual} is below minimum {min}")]
    BelowMin {
        actual: crate::domain_types::BoundedLen,
        min: crate::domain_types::BoundedLen,
    },
    #[error("bounded value minimum {min} exceeds maximum {max}")]
    InvalidBounds {
        min: crate::domain_types::BoundedLen,
        max: crate::domain_types::BoundedLen,
    },
}
