#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynEndpointRegistryState(syn::Type);
