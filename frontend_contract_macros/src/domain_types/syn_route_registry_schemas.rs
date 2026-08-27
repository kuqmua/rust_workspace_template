#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistrySchemas(Vec<syn::Type>);
impl SynRouteRegistrySchemas {
    pub(crate) fn into_inner(self) -> Vec<syn::Type> {
        self.0
    }
}
