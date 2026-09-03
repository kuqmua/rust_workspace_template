#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
    proc_macro_newtype_foundation_foundation_to_tokens::ToTokens,
)]
pub(crate) struct ProcMacro2GeneratedTokenStream(proc_macro2::TokenStream);
impl From<ProcMacro2GeneratedTokenStream> for proc_macro2::TokenStream {
    fn from(proc_macro2_generated_token_stream: ProcMacro2GeneratedTokenStream) -> Self {
        proc_macro2_generated_token_stream.0
    }
}
