#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_to_tokens::ToTokens,
)]
pub(crate) struct ProcMacro2GeneratedNamingTokenStream(proc_macro2::TokenStream);
