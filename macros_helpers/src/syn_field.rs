#[derive(Debug, Clone, optml::Optml)]
pub struct SynField {
    pub ident: SynFieldIdent,
    pub type0: SynFieldType,
    pub vis: SynFieldVis,
}
#[derive(Debug, Clone, PartialEq, newtype::Newtype)]
#[newtype(as_ref_owned, display, from_inner, to_tokens)]
pub struct SynFieldIdent(syn::Ident);
impl std::ops::Deref for SynFieldIdent {
    type Target = syn::Ident;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner, to_tokens)]
pub struct SynFieldType(syn::Type);
impl std::ops::Deref for SynFieldType {
    type Target = syn::Type;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner, to_tokens)]
pub struct SynFieldVis(syn::Visibility);
impl std::ops::Deref for SynFieldVis {
    type Target = syn::Visibility;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
