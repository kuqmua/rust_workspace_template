#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryFamily(syn::Type);
impl SynRouteRegistryFamily {
    pub(crate) fn into_inner(self) -> syn::Type {
        self.0
    }
}
