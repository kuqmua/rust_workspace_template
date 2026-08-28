#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct SynType(syn::Type);
