use super::{SingleFlightKey, TokioSingleFlightSender};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub(super) struct SingleFlightInner {
    pub(super) flights: bounded_types::domain_types::hash::BoundedHashMap<
        SingleFlightKey,
        TokioSingleFlightSender,
        { usize::MAX },
    >,
}
