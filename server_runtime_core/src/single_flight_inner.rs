#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct SingleFlightInner(
    bounded_types::bounded_hash_map::BoundedHashMap<
        crate::single_flight_key::SingleFlightKey,
        crate::tokio_single_flight_sender::TokioSingleFlightSender,
        { usize::MAX },
    >,
);
