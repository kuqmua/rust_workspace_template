#[derive(Debug, Clone, optml::Optml)]
pub struct SynField {
    pub identifier: SynFieldIdentifier,
    pub type0: SynFieldType,
    pub vis: SynFieldVis,
}
#[derive(
    optml::Optml,
    Debug,
    Clone,
    PartialEq,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynFieldIdentifier(syn::Ident);
#[derive(
    optml::Optml,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynFieldType(syn::Type);
#[derive(
    optml::Optml,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynFieldVis(syn::Visibility);
