#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub(crate) struct ProcMacro2GeneratedNamingTokenStream(proc_macro2::TokenStream);
