#[derive(Debug, Clone, optml::Optml)]
pub struct SynField {
    pub identifier: SynFieldIdentifier,
    pub type0: SynFieldType,
    pub vis: SynFieldVis,
}
#[derive(Debug, Clone, PartialEq, newtype::Newtype)]
#[newtype(as_ref_owned, deref_inner, display, from_inner, to_tokens)]
pub struct SynFieldIdentifier(syn::Ident);
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(as_ref_owned, deref_inner, from_inner, to_tokens)]
pub struct SynFieldType(syn::Type);
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(as_ref_owned, deref_inner, from_inner, to_tokens)]
pub struct SynFieldVis(syn::Visibility);
