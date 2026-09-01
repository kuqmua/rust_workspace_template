#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype_foundation::FromInner,
)]
pub(crate) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
impl AsRef<syn::Ident> for SynIdentifierRef<'_> {
    fn as_ref(&self) -> &syn::Ident {
        self.0
    }
}
