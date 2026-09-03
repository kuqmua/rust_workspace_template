#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct ProcMacro2TokensMut<'tokens_lt>(&'tokens_lt mut proc_macro2::TokenStream);

impl ProcMacro2TokensMut<'_> {
    pub(crate) fn append(&mut self, part: impl quote::ToTokens) {
        part.to_tokens(self.0);
    }
}
