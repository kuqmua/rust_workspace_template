#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::DerefTarget,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>(pub(super) &'lt proc_macro2::TokenStream);
