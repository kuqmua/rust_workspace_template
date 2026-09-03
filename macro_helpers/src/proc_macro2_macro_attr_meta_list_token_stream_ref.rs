#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_to_tokens::ToTokens,
)]
pub struct ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>(&'lt proc_macro2::TokenStream);
