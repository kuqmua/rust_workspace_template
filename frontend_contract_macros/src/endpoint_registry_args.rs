use crate::{SynEndpointRegistryBindings, SynEndpointRegistryState};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryArgs {
    bindings: SynEndpointRegistryBindings,
    state: SynEndpointRegistryState,
}
