#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = constants_str::ALLOWED_HTTP_ORIGIN_LIST_IS_INVALID)]
pub struct AllowedOriginsError;

impl From<bounded_types::BoundedValueError> for AllowedOriginsError {
    fn from(_value: bounded_types::BoundedValueError) -> Self {
        Self
    }
}
