#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynFieldType(syn::Type);
