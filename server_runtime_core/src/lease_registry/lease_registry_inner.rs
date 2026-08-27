use super::{LeaseEntry, LeaseId, LeaseKey};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub(super) struct LeaseRegistryInner {
    pub(super) by_id:
        bounded_types::domain_types::hash::BoundedHashMap<LeaseId, LeaseEntry, { usize::MAX }>,
    pub(super) by_key:
        bounded_types::domain_types::hash::BoundedHashMap<LeaseKey, LeaseId, { usize::MAX }>,
}
