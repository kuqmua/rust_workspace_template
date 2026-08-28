#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct SynIdentifier(syn::Ident);
impl From<syn::Ident> for SynIdentifier {
    fn from(value: syn::Ident) -> Self {
        Self(value)
    }
}
impl SynIdentifier {
    pub(crate) fn into_inner(self) -> syn::Ident {
        self.0
    }
}
