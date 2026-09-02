#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::ToTokens,
)]
pub(crate) struct ProcMacro2GeneratedTokenStream(proc_macro2::TokenStream);
impl From<ProcMacro2GeneratedTokenStream> for proc_macro2::TokenStream {
    fn from(proc_macro2_generated_token_stream: ProcMacro2GeneratedTokenStream) -> Self {
        proc_macro2_generated_token_stream.0
    }
}
impl From<ProcMacro2GeneratedTokenStream> for proc_macro::TokenStream {
    fn from(proc_macro2_generated_token_stream: ProcMacro2GeneratedTokenStream) -> Self {
        proc_macro2::TokenStream::from(proc_macro2_generated_token_stream).into()
    }
}
