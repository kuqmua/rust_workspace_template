#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Default)]
pub struct ProcMacro2MacroTokens(Vec<proc_macro2::TokenTree>);
impl From<proc_macro2::TokenStream> for ProcMacro2MacroTokens {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value.into_iter().collect())
    }
}
impl From<ProcMacro2MacroTokens> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2MacroTokens) -> Self {
        value.0.into_iter().collect()
    }
}
impl ProcMacro2MacroTokens {
    pub fn from_into<T>(value: T) -> Self
    where
        T: Into<proc_macro2::TokenStream>,
    {
        Self::from(value.into())
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.iter().cloned());
    }
}
impl std::fmt::Display for ProcMacro2MacroTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .cloned()
            .collect::<proc_macro2::TokenStream>()
            .fmt(f)
    }
}
impl syn::parse::Parse for ProcMacro2MacroTokens {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        input.step(|cursor| {
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
