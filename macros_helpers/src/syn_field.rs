#[derive(Debug, Clone, optml::Optml)]
pub struct SynField {
    pub ident: SynFieldIdent,
    pub type0: SynFieldType,
    pub vis: SynFieldVis,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SynFieldIdent(syn::Ident);
impl From<syn::Ident> for SynFieldIdent {
    fn from(value: syn::Ident) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Ident> for SynFieldIdent {
    fn as_ref(&self) -> &syn::Ident {
        &self.0
    }
}
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
#[derive(Debug, Clone)]
pub struct SynFieldType(syn::Type);
impl From<syn::Type> for SynFieldType {
    fn from(value: syn::Type) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Type> for SynFieldType {
    fn as_ref(&self) -> &syn::Type {
        &self.0
    }
}
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
#[derive(Debug, Clone)]
pub struct SynFieldVis(syn::Visibility);
impl From<syn::Visibility> for SynFieldVis {
    fn from(value: syn::Visibility) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Visibility> for SynFieldVis {
    fn as_ref(&self) -> &syn::Visibility {
        &self.0
    }
}
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
