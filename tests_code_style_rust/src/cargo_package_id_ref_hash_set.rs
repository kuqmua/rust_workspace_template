#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct CargoPackageIdRefHashSet<'metadata_lt>(
    std::collections::HashSet<&'metadata_lt cargo_metadata::PackageId>,
);
