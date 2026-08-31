#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(super) struct ServiceCatalogEntriesRef<'entries_lt>(
    &'entries_lt [crate::service_catalog_entry::ServiceCatalogEntry],
);
