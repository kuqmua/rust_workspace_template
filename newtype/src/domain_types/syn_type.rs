#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct SynType(syn::Type);
impl From<syn::Type> for SynType {
    fn from(value: syn::Type) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Type> for SynType {
    fn as_ref(&self) -> &syn::Type {
        &self.0
    }
}
