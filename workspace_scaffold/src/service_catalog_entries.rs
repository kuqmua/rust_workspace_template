#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(super) struct ServiceCatalogEntries(
    bounded_types::bounded_vec::BoundedVec<
        crate::service_catalog_entry::ServiceCatalogEntry,
        0,
        { usize::MAX },
    >,
);
