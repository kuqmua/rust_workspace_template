#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub(super) struct LeaseRegistryInner {
    pub(super) by_id: bounded_types::bounded_hash_map::BoundedHashMap<
        crate::lease_id::LeaseId,
        crate::lease_entry::LeaseEntry,
        { usize::MAX },
    >,
    pub(super) by_key: bounded_types::bounded_hash_map::BoundedHashMap<
        crate::lease_key::LeaseKey,
        crate::lease_id::LeaseId,
        { usize::MAX },
    >,
}
