#[derive(Debug, Clone, Default)]
pub struct GeneratedRustTs(pub proc_macro2::TokenStream);
impl From<proc_macro2::TokenStream> for GeneratedRustTs {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
impl quote::ToTokens for GeneratedRustTs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
impl std::fmt::Display for GeneratedRustTs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::ops::Deref for GeneratedRustTs {
    type Target = proc_macro2::TokenStream;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
