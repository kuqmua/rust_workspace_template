#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryState(syn::Type);
impl SynRouteRegistryState {
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}
