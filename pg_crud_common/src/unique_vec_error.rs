#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum UniqueVecError {
    #[error("{} {max}", constants_str::BOUNDED_UNIQUE_VEC_ABOVE_MAX)]
    AboveMax {
        max: crate::unique_vec_len::UniqueVecLen,
    },
    #[error("{}: {actual} < {min}", constants_str::BOUNDED_UNIQUE_VEC_BELOW_MIN)]
    BelowMin {
        actual: crate::unique_vec_len::UniqueVecLen,
        min: crate::unique_vec_len::UniqueVecLen,
    },
    #[error("{}", constants_str::BOUNDED_UNIQUE_VEC_DUPLICATE)]
    Duplicate,
    #[error("{}: {min} > {max}", constants_str::BOUNDED_UNIQUE_VEC_INVALID_BOUNDS)]
    InvalidBounds {
        min: crate::unique_vec_len::UniqueVecLen,
        max: crate::unique_vec_len::UniqueVecLen,
    },
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for UniqueVecError {
    fn from(value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        match value {
            bounded_types::bounded_value_error::BoundedValueError::AboveMax { max, .. } => {
                Self::AboveMax {
                    max: crate::unique_vec_len::UniqueVecLen::from(max.get()),
                }
            }
            bounded_types::bounded_value_error::BoundedValueError::BelowMin { actual, min } => {
                Self::BelowMin {
                    actual: crate::unique_vec_len::UniqueVecLen::from(actual.get()),
                    min: crate::unique_vec_len::UniqueVecLen::from(min.get()),
                }
            }
            bounded_types::bounded_value_error::BoundedValueError::InvalidBounds { min, max } => {
                Self::InvalidBounds {
                    min: crate::unique_vec_len::UniqueVecLen::from(min.get()),
                    max: crate::unique_vec_len::UniqueVecLen::from(max.get()),
                }
            }
        }
    }
}
