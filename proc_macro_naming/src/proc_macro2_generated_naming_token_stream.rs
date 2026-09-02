#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::ToTokens,
)]
pub(crate) struct ProcMacro2GeneratedNamingTokenStream(proc_macro2::TokenStream);
