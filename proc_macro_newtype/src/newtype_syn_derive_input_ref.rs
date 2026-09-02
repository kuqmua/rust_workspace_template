#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_foundation::AsRefInner,
    proc_macro_newtype_foundation::FromInner,
)]
pub(crate) struct NewtypeSynDeriveInputRef<'syn_lt>(&'syn_lt syn::DeriveInput);
impl<'syn_lt> From<NewtypeSynDeriveInputRef<'syn_lt>> for &'syn_lt syn::DeriveInput {
    fn from(newtype_syn_derive_input_ref: NewtypeSynDeriveInputRef<'syn_lt>) -> Self {
        newtype_syn_derive_input_ref.0
    }
}
