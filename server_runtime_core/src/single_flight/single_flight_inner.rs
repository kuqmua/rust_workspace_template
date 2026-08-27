#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{SingleFlightKey, TokioSingleFlightSender};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub(super) struct SingleFlightInner {
    pub(super) flights: bounded_types::domain_types::hash::BoundedHashMap<
        SingleFlightKey,
        TokioSingleFlightSender,
        { usize::MAX },
    >,
}
