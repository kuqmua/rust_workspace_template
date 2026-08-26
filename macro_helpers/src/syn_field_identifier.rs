#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynFieldIdentifier(syn::Ident);
