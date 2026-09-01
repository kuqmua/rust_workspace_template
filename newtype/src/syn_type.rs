#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype_foundation::FromInner)]
pub(crate) struct SynType(syn::Type);
impl AsRef<syn::Type> for SynType {
    fn as_ref(&self) -> &syn::Type {
        &self.0
    }
}
