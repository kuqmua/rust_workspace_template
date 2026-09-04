#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(bare)]
pub struct HttpContractObservation {
    #[constructor(order = 2)]
    body: crate::http_contract_body::HttpContractBody,
    #[constructor(order = 0)]
    #[getters(copy)]
    metadata: frontend_contract::route_metadata::RouteMetadata,
    #[constructor(order = 1)]
    #[getters(copy)]
    status: crate::http_contract_status::HttpContractStatus,
}
