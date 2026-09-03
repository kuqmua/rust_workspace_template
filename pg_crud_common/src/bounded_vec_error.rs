#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
#[allow(
    clippy::module_name_repetitions,
    reason = "lint suppression is required here"
)]
pub enum BoundedVecError {
    #[error("bounded vector length {actual} exceeds limit {max}")]
    AboveMax {
        actual: crate::pg_bounded_vec_len::PgBoundedVecLen,
        max: crate::pg_bounded_vec_len::PgBoundedVecLen,
    },
    #[error("bounded vector length {actual} is below minimum {min}")]
    BelowMin {
        actual: crate::pg_bounded_vec_len::PgBoundedVecLen,
        min: crate::pg_bounded_vec_len::PgBoundedVecLen,
    },
    #[error("bounded vector minimum {min} exceeds maximum {max}")]
    InvalidBounds {
        min: crate::pg_bounded_vec_len::PgBoundedVecLen,
        max: crate::pg_bounded_vec_len::PgBoundedVecLen,
    },
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for BoundedVecError {
    fn from(bounded_value_error: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        match bounded_value_error {
            bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual, max } => {
                Self::AboveMax {
                    actual: crate::pg_bounded_vec_len::PgBoundedVecLen::from(actual.get()),
                    max: crate::pg_bounded_vec_len::PgBoundedVecLen::from(max.get()),
                }
            }
            bounded_types::bounded_value_error::BoundedValueError::BelowMin { actual, min } => {
                Self::BelowMin {
                    actual: crate::pg_bounded_vec_len::PgBoundedVecLen::from(actual.get()),
                    min: crate::pg_bounded_vec_len::PgBoundedVecLen::from(min.get()),
                }
            }
            bounded_types::bounded_value_error::BoundedValueError::InvalidBounds { min, max } => {
                Self::InvalidBounds {
                    min: crate::pg_bounded_vec_len::PgBoundedVecLen::from(min.get()),
                    max: crate::pg_bounded_vec_len::PgBoundedVecLen::from(max.get()),
                }
            }
        }
    }
}
