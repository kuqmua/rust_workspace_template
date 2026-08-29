#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryArgs {
    bindings: crate::syn_endpoint_registry_bindings::SynEndpointRegistryBindings,
    state: crate::syn_endpoint_registry_state::SynEndpointRegistryState,
}
