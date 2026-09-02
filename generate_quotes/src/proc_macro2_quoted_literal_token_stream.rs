#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::ToTokens,
)]
pub struct ProcMacro2QuotedLiteralTokenStream(proc_macro2::TokenStream);
