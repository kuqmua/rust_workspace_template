#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
impl<'syn_lt> From<&'syn_lt syn::Type> for SynTypeRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Type) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Type> for SynTypeRef<'_> {
    fn as_ref(&self) -> &syn::Type {
        self.0
    }
}
