use super::ServiceCatalogEntry;
use bounded_types::domain_types::vector::BoundedVec;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct ServiceCatalogEntries(
    pub(super) BoundedVec<ServiceCatalogEntry, 0, { usize::MAX }>,
);
