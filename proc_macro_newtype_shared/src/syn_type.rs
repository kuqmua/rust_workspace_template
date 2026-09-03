#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_foundation_foundation_as_ref_inner::AsRefInner,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
)]
pub(crate) struct SynType(syn::Type);
