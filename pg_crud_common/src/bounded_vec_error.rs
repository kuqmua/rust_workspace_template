#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::module_name_repetitions)] // callers need an unambiguous error name in public signatures
pub enum BoundedVecError {
    #[error("bounded vector length {actual} exceeds limit {max}")]
    AboveMax {
        actual: super::BoundedVecLen,
        max: super::BoundedVecLen,
    },
    #[error("bounded vector length {actual} is below minimum {min}")]
    BelowMin {
        actual: super::BoundedVecLen,
        min: super::BoundedVecLen,
    },
    #[error("bounded vector minimum {min} exceeds maximum {max}")]
    InvalidBounds {
        min: super::BoundedVecLen,
        max: super::BoundedVecLen,
    },
}

impl From<bounded_types::BoundedValueError> for BoundedVecError {
    fn from(value: bounded_types::BoundedValueError) -> Self {
        match value {
            bounded_types::BoundedValueError::AboveMax { actual, max } => Self::AboveMax {
                actual: super::BoundedVecLen::from(actual.get()),
                max: super::BoundedVecLen::from(max.get()),
            },
            bounded_types::BoundedValueError::BelowMin { actual, min } => Self::BelowMin {
                actual: super::BoundedVecLen::from(actual.get()),
                min: super::BoundedVecLen::from(min.get()),
            },
            bounded_types::BoundedValueError::InvalidBounds { min, max } => Self::InvalidBounds {
                min: super::BoundedVecLen::from(min.get()),
                max: super::BoundedVecLen::from(max.get()),
            },
        }
    }
}
