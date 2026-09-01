#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype_foundation::AsRefInner,
    newtype_foundation::FromInner,
)]
pub(crate) struct SynType(syn::Type);
