#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryBinding {
    contract: crate::syn_endpoint_registry_contract::SynEndpointRegistryContract,
    endpoint: crate::syn_endpoint_registry_endpoint::SynEndpointRegistryEndpoint,
}
