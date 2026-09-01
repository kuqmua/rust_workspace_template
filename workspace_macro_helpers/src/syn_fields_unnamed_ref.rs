#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype_foundation::FromInner,
    newtype_foundation::GetInner,
)]
pub struct SynFieldsUnnamedRef<'fields_lt>(&'fields_lt syn::FieldsUnnamed);
impl std::ops::Deref for SynFieldsUnnamedRef<'_> {
    type Target = syn::FieldsUnnamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
