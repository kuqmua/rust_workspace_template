#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynIdent(syn::Ident);
impl SynIdent {
    pub(crate) fn into_inner(self) -> syn::Ident {
        self.0
    }
}
