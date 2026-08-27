use super::{LeaseKey, LeaseState, TokioLeaseInstant};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct LeaseEntry {
    pub(super) heartbeat: TokioLeaseInstant,
    pub(super) key: LeaseKey,
    pub(super) state: LeaseState,
}
