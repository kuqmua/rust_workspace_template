#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct ProcMacro2GeneratedTokenStream(proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for ProcMacro2GeneratedTokenStream {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
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
impl quote::ToTokens for ProcMacro2GeneratedTokenStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
