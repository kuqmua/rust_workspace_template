#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::FromInner,
)]
pub(super) struct ServiceCatalogEntries(
    bounded_types::bounded_vec::BoundedVec<
        crate::service_catalog_entry::ServiceCatalogEntry,
        0,
        { usize::MAX },
    >,
);
