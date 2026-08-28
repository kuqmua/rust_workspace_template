#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("trusted proxy range list exceeds its maximum item count")]
pub struct TrustedProxyRangesError;

impl From<bounded_types::BoundedValueError> for TrustedProxyRangesError {
    fn from(_value: bounded_types::BoundedValueError) -> Self {
        Self
    }
}
