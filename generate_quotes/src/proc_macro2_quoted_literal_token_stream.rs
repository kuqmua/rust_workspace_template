#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_to_tokens::ToTokens,
)]
pub struct ProcMacro2QuotedLiteralTokenStream(proc_macro2::TokenStream);
