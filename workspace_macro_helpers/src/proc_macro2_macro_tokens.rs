#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Default)]
pub struct ProcMacro2MacroTokens(Vec<proc_macro2::TokenTree>);
impl From<proc_macro2::TokenStream> for ProcMacro2MacroTokens {
    fn from(token_stream: proc_macro2::TokenStream) -> Self {
        Self(token_stream.into_iter().collect())
    }
}
impl From<ProcMacro2MacroTokens> for proc_macro2::TokenStream {
    fn from(proc_macro2_macro_tokens: ProcMacro2MacroTokens) -> Self {
        proc_macro2_macro_tokens.0.into_iter().collect()
    }
}
impl ProcMacro2MacroTokens {
    pub fn from_into<T>(t: T) -> Self
    where
        T: Into<proc_macro2::TokenStream>,
    {
        Self::from(t.into())
    }
    #[must_use]
    pub fn into_inner(self) -> proc_macro2::TokenStream {
        self.0.into_iter().collect()
    }
}
impl std::ops::Deref for ProcMacro2MacroTokens {
    type Target = [proc_macro2::TokenTree];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl IntoIterator for ProcMacro2MacroTokens {
    type IntoIter = std::vec::IntoIter<proc_macro2::TokenTree>;
    type Item = proc_macro2::TokenTree;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl quote::ToTokens for ProcMacro2MacroTokens {
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        token_stream.extend(self.0.iter().cloned());
    }
}
impl std::fmt::Display for ProcMacro2MacroTokens {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .cloned()
            .collect::<proc_macro2::TokenStream>()
            .fmt(formatter)
    }
}
impl syn::parse::Parse for ProcMacro2MacroTokens {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        parse_stream.step(|cursor| {
            let mut rest = *cursor;
            let mut tokens = proc_macro2::TokenStream::new();
            while let Some((token, next)) = rest.token_tree() {
                tokens.extend([token]);
                rest = next;
            }
            Ok((Self::from(tokens), rest))
        })
    }
}
