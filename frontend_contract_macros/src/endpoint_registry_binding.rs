use super::{SynEndpointRegistryContract, SynEndpointRegistryEndpoint};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryBinding {
    contract: SynEndpointRegistryContract,
    endpoint: SynEndpointRegistryEndpoint,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl EndpointRegistryBinding {
    #[allow(
        clippy::single_call_fn,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        contract: SynEndpointRegistryContract,
        endpoint: SynEndpointRegistryEndpoint,
    ) -> Self {
        Self { contract, endpoint }
    }
}
