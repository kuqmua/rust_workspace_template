#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::ToTokens,
)]
pub struct ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>(&'lt proc_macro2::TokenStream);
