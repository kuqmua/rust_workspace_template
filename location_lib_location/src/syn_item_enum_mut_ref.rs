#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(crate) struct SynItemEnumMutRef<'item_lt>(&'item_lt mut syn::ItemEnum);

impl<'item_lt> SynItemEnumMutRef<'item_lt> {
    pub(crate) const fn into_inner(self) -> &'item_lt mut syn::ItemEnum {
        self.0
    }
}
