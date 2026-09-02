#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum TrustedProxyRangesError {
    #[error("trusted proxy range list exceeds its maximum item count")]
    TooMany,
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for TrustedProxyRangesError {
    fn from(bounded_value_error: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = bounded_value_error;
        Self::TooMany
    }
}
