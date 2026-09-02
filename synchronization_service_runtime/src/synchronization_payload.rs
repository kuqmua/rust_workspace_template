#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
)]
pub struct SynchronizationPayload(
    bounded_types::bounded_vec::BoundedVec<
        u8,
        0,
        { super::synchronization_payload_max_bytes::SYNCHRONIZATION_PAYLOAD_MAX_BYTES },
    >,
);

impl TryFrom<Vec<u8>> for SynchronizationPayload {
    type Error = crate::synchronization_payload_too_large::SynchronizationPayloadTooLarge;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self)
            .map_err(crate::synchronization_payload_too_large::SynchronizationPayloadTooLarge::from)
    }
}
