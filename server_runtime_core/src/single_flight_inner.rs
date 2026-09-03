#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct SingleFlightInner(
    bounded_types::bounded_hash_map::BoundedHashMap<
        crate::single_flight_key::SingleFlightKey,
        crate::tokio_single_flight_sender::TokioSingleFlightSender,
        { usize::MAX },
    >,
);
