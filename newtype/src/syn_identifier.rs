#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct SynIdentifier(syn::Ident);
impl From<syn::Ident> for SynIdentifier {
    fn from(value: syn::Ident) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Ident> for SynIdentifier {
    fn as_ref(&self) -> &syn::Ident {
        &self.0
    }
}
