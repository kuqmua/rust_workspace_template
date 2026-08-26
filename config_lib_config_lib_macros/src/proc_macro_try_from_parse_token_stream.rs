#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct ProcMacroTryFromParseTokenStream(proc_macro::TokenStream);
