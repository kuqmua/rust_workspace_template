#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynMacroAttrRef<'lt>(pub(super) &'lt syn::Attribute);
