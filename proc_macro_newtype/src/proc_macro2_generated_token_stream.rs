#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::ToTokens,
)]
pub(crate) struct ProcMacro2GeneratedTokenStream(proc_macro2::TokenStream);
impl From<ProcMacro2GeneratedTokenStream> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2GeneratedTokenStream) -> Self {
        value.0
    }
}
impl From<ProcMacro2GeneratedTokenStream> for proc_macro::TokenStream {
    fn from(value: ProcMacro2GeneratedTokenStream) -> Self {
        proc_macro2::TokenStream::from(value).into()
    }
}
