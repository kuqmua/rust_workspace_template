#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynFieldVis(syn::Visibility);
