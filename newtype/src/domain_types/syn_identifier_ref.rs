#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
impl<'syn_lt> From<&'syn_lt syn::Ident> for SynIdentifierRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Ident) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Ident> for SynIdentifierRef<'_> {
    fn as_ref(&self) -> &syn::Ident {
        self.0
    }
}
