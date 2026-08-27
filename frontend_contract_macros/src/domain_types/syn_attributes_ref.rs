#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct SynAttributesRef<'attributes_lt>(&'attributes_lt [syn::Attribute]);
impl<'attributes_lt> SynAttributesRef<'attributes_lt> {
    pub(crate) const fn get(self) -> &'attributes_lt [syn::Attribute] {
        self.0
    }
}
