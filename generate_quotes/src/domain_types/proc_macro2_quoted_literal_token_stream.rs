#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub struct ProcMacro2QuotedLiteralTokenStream(proc_macro2::TokenStream);
