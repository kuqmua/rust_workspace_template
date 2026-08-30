#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AllowedOriginsError {
    #[error("{message}", message = constants_str::catalog::ALLOWED_HTTP_ORIGIN_LIST_IS_INVALID)]
    Invalid,
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for AllowedOriginsError {
    fn from(_value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        Self::Invalid
    }
}
