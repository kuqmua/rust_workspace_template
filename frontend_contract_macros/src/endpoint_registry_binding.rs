#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryBinding {
    contract: crate::syn_endpoint_registry_contract::SynEndpointRegistryContract,
    endpoint: crate::syn_endpoint_registry_endpoint::SynEndpointRegistryEndpoint,
}
