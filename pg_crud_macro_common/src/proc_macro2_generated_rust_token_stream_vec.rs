#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Default,
    proc_macro_newtype::FromInner,
)]
pub struct ProcMacro2GeneratedRustTokenStreamVec(
    Vec<macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream>,
);
impl quote::ToTokens for ProcMacro2GeneratedRustTokenStreamVec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0
            .iter()
            .for_each(|element| quote::ToTokens::to_tokens(element, tokens));
    }
}
impl
    FromIterator<
        macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    > for ProcMacro2GeneratedRustTokenStreamVec
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<
            Item = macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
        >,
    {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}
