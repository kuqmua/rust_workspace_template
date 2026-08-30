#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct ContractSynIdent(syn::Ident);
impl ContractSynIdent {
    pub(crate) fn into_inner(self) -> syn::Ident {
        self.0
    }
}
