#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub(super) struct ServiceCatalogEntriesRef<'entries_lt>(
    &'entries_lt [crate::service_catalog_entry::ServiceCatalogEntry],
);
