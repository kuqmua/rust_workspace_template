#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype_foundation::FromInner,
    newtype_foundation::GetInner,
)]
pub struct SynFieldsNamedRef<'fields_lt>(&'fields_lt syn::FieldsNamed);
impl std::ops::Deref for SynFieldsNamedRef<'_> {
    type Target = syn::FieldsNamed;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
