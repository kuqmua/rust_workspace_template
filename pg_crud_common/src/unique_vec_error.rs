#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum UniqueVecError {
    #[error("{} {max}", constants_str::BOUNDED_UNIQUE_VEC_ABOVE_MAX)]
    AboveMax { max: super::UniqueVecLen },
    #[error("{}: {actual} < {min}", constants_str::BOUNDED_UNIQUE_VEC_BELOW_MIN)]
    BelowMin {
        actual: super::UniqueVecLen,
        min: super::UniqueVecLen,
    },
    #[error("{}", constants_str::BOUNDED_UNIQUE_VEC_DUPLICATE)]
    Duplicate,
    #[error("{}: {min} > {max}", constants_str::BOUNDED_UNIQUE_VEC_INVALID_BOUNDS)]
    InvalidBounds {
        min: super::UniqueVecLen,
        max: super::UniqueVecLen,
    },
}

impl From<bounded_types::domain_types::BoundedValueError> for UniqueVecError {
    fn from(value: bounded_types::domain_types::BoundedValueError) -> Self {
        match value {
            bounded_types::domain_types::BoundedValueError::AboveMax { max, .. } => {
                Self::AboveMax {
                    max: super::UniqueVecLen::from(max.get()),
                }
            }
            bounded_types::domain_types::BoundedValueError::BelowMin { actual, min } => {
                Self::BelowMin {
                    actual: super::UniqueVecLen::from(actual.get()),
                    min: super::UniqueVecLen::from(min.get()),
                }
            }
            bounded_types::domain_types::BoundedValueError::InvalidBounds { min, max } => {
                Self::InvalidBounds {
                    min: super::UniqueVecLen::from(min.get()),
                    max: super::UniqueVecLen::from(max.get()),
                }
            }
        }
    }
}
