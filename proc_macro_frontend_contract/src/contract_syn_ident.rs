#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::AsRefOwned,
)]
pub(crate) struct ContractSynIdent(syn::Ident);
impl ContractSynIdent {
    pub(crate) fn into_inner(self) -> syn::Ident {
        self.0
    }
}
