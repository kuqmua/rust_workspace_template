#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", std::any::type_name::<Self>())]
pub struct SynchronizationPayloadTooLarge;

impl From<bounded_types::domain_types::BoundedValueError> for SynchronizationPayloadTooLarge {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
