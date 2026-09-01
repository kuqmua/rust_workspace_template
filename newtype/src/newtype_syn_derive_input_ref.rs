#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype_foundation::FromInner,
)]
pub(crate) struct NewtypeSynDeriveInputRef<'syn_lt>(&'syn_lt syn::DeriveInput);
impl AsRef<syn::DeriveInput> for NewtypeSynDeriveInputRef<'_> {
    fn as_ref(&self) -> &syn::DeriveInput {
        self.0
    }
}
impl<'syn_lt> From<NewtypeSynDeriveInputRef<'syn_lt>> for &'syn_lt syn::DeriveInput {
    fn from(value: NewtypeSynDeriveInputRef<'syn_lt>) -> Self {
        value.0
    }
}
