use super::{SynEndpointRegistryBindings, SynEndpointRegistryState};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryArgs {
    bindings: SynEndpointRegistryBindings,
    state: SynEndpointRegistryState,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl EndpointRegistryArgs {
    #[allow(
        clippy::single_call_fn,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        bindings: SynEndpointRegistryBindings,
        state: SynEndpointRegistryState,
    ) -> Self {
        Self { bindings, state }
    }
}
