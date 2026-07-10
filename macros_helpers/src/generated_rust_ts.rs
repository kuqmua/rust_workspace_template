#[derive(Debug, Clone, Default, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct GeneratedRustTs(proc_macro2::TokenStream);
impl From<GeneratedRustTs> for proc_macro2::TokenStream {
    fn from(value: GeneratedRustTs) -> Self {
        value.0
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
