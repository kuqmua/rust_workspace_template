use super::ServiceCatalogEntry;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct ServiceCatalogEntriesRef<'entries_lt>(
    pub(super) &'entries_lt [ServiceCatalogEntry],
);
