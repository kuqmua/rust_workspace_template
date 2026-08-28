use super::{SynEndpointRegistryContract, SynEndpointRegistryEndpoint};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct EndpointRegistryBinding {
    pub(crate) contract: SynEndpointRegistryContract,
    pub(crate) endpoint: SynEndpointRegistryEndpoint,
}
