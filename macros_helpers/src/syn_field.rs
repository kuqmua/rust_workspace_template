#[derive(Debug, Clone, optml::Optml)]
pub struct SynField {
    pub ident: SynFieldIdent,
    pub type0: SynFieldType,
    pub vis: SynFieldVis,
}
#[derive(Debug, Clone, PartialEq, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct SynFieldIdent(syn::Ident);
impl std::ops::Deref for SynFieldIdent {
    type Target = syn::Ident;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl quote::ToTokens for SynFieldIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
impl std::fmt::Display for SynFieldIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct SynFieldType(syn::Type);
impl std::ops::Deref for SynFieldType {
    type Target = syn::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl quote::ToTokens for SynFieldType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct SynFieldVis(syn::Visibility);
impl std::ops::Deref for SynFieldVis {
    type Target = syn::Visibility;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl quote::ToTokens for SynFieldVis {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
