#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::GetInner,
)]
pub struct SynFieldsUnnamedRef<'fields_lt>(&'fields_lt syn::FieldsUnnamed);
impl std::ops::Deref for SynFieldsUnnamedRef<'_> {
    type Target = syn::FieldsUnnamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
