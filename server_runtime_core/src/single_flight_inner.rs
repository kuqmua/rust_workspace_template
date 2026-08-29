#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub(super) struct SingleFlightInner {
    pub(super) flights: bounded_types::bounded_hash_map::BoundedHashMap<
        crate::single_flight_key::SingleFlightKey,
        crate::tokio_single_flight_sender::TokioSingleFlightSender,
        { usize::MAX },
    >,
}
