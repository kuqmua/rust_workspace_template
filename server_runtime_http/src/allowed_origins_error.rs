#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum AllowedOriginsError {
    #[error("{message}", message = constants_str::ALLOWED_HTTP_ORIGIN_LIST_IS_INVALID)]
    Invalid,
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for AllowedOriginsError {
    fn from(value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = value;
        Self::Invalid
    }
}
