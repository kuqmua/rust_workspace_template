#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct ProcMacro2TryFromParseInput(proc_macro2::TokenStream);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct ProcMacro2TryFromParseFixedErrorTy(Option<proc_macro2::TokenStream>);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct ProcMacroTryFromParseTokenStream(proc_macro::TokenStream);
