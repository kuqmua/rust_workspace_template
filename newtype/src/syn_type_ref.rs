#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype_foundation::FromInner,
)]
pub(crate) struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
impl AsRef<syn::Type> for SynTypeRef<'_> {
    fn as_ref(&self) -> &syn::Type {
        self.0
    }
}
