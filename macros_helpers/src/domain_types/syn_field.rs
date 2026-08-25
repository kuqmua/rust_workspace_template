#[derive(Debug, Clone, optimal_memory_layout::OptimalMemoryLayout)]
pub struct SynField {
    pub identifier: SynFieldIdentifier,
    pub type0: SynFieldType,
    pub vis: SynFieldVis,
}
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
