#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{LeaseKey, LeaseState, TokioLeaseInstant};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(super) struct LeaseEntry {
    pub(super) heartbeat: TokioLeaseInstant,
    pub(super) key: LeaseKey,
    pub(super) state: LeaseState,
}
