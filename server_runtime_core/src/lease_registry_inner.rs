#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{LeaseEntry, LeaseId, LeaseKey};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub(super) struct LeaseRegistryInner {
    pub(super) by_id: bounded_types::BoundedHashMap<LeaseId, LeaseEntry, { usize::MAX }>,
    pub(super) by_key: bounded_types::BoundedHashMap<LeaseKey, LeaseId, { usize::MAX }>,
}
