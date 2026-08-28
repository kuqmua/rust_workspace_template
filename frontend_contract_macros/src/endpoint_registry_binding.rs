use crate::{SynEndpointRegistryContract, SynEndpointRegistryEndpoint};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryBinding {
    contract: SynEndpointRegistryContract,
    endpoint: SynEndpointRegistryEndpoint,
}
