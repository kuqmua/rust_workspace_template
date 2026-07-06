#[derive(Debug, Clone, optml::Optml)]
pub struct SynField {
    pub ident: syn::Ident,
    pub type0: syn::Type,
    pub vis: syn::Visibility,
}
