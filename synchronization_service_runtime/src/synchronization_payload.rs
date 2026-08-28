#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct SynchronizationPayload(
    bounded_types::BoundedVec<
        u8,
        0,
        { super::synchronization_payload_max_bytes::SYNCHRONIZATION_PAYLOAD_MAX_BYTES },
    >,
);

impl TryFrom<Vec<u8>> for SynchronizationPayload {
    type Error = super::SynchronizationPayloadTooLarge;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(super::SynchronizationPayloadTooLarge::from)
    }
}
