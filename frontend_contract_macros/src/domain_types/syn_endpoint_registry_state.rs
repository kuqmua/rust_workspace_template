#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynEndpointRegistryState(syn::Type);
impl SynEndpointRegistryState {
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}
