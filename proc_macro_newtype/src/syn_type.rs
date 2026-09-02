#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_foundation::AsRefInner,
    proc_macro_newtype_foundation::FromInner,
)]
pub(crate) struct SynType(syn::Type);
