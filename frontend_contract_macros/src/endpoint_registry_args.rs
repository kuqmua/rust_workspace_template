use super::{SynEndpointRegistryBindings, SynEndpointRegistryState};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct EndpointRegistryArgs {
    pub(crate) bindings: SynEndpointRegistryBindings,
    pub(crate) state: SynEndpointRegistryState,
}
