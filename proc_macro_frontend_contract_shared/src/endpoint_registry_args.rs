#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryArgs {
    bindings: crate::syn_endpoint_registry_bindings::SynEndpointRegistryBindings,
    state: crate::syn_endpoint_registry_state::SynEndpointRegistryState,
}
