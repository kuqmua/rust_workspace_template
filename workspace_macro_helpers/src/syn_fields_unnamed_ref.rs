#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
    proc_macro_newtype_foundation_foundation_get_inner::GetInner,
)]
pub struct SynFieldsUnnamedRef<'fields_lt>(&'fields_lt syn::FieldsUnnamed);
impl std::ops::Deref for SynFieldsUnnamedRef<'_> {
    type Target = syn::FieldsUnnamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
