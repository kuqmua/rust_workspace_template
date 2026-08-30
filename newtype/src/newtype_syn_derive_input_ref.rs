#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct NewtypeSynDeriveInputRef<'syn_lt>(&'syn_lt syn::DeriveInput);
impl<'syn_lt> From<&'syn_lt syn::DeriveInput> for NewtypeSynDeriveInputRef<'syn_lt> {
    fn from(value: &'syn_lt syn::DeriveInput) -> Self {
        Self(value)
    }
}
impl AsRef<syn::DeriveInput> for NewtypeSynDeriveInputRef<'_> {
    fn as_ref(&self) -> &syn::DeriveInput {
        self.0
    }
}
impl<'syn_lt> NewtypeSynDeriveInputRef<'syn_lt> {
    pub(crate) const fn get(self) -> &'syn_lt syn::DeriveInput {
        self.0
    }
}
