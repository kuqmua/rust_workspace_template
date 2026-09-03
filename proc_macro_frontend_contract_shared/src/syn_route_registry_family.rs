#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
)]
pub(crate) struct SynRouteRegistryFamily(syn::Type);
