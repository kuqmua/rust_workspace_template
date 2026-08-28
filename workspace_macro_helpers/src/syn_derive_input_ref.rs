#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct SynDeriveInputRef<'input_lt>(&'input_lt syn::DeriveInput);
impl<'input_lt> From<&'input_lt syn::DeriveInput> for SynDeriveInputRef<'input_lt> {
    fn from(value: &'input_lt syn::DeriveInput) -> Self {
        Self(value)
    }
}
impl<'input_lt> SynDeriveInputRef<'input_lt> {
    #[must_use]
    pub const fn get(self) -> &'input_lt syn::DeriveInput {
        self.0
    }
}
