#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct HttpContractObservation {
    #[constructor(order = 2)]
    body: crate::http_contract_body::HttpContractBody,
    #[constructor(order = 0)]
    metadata: frontend_contract::route_metadata::RouteMetadata,
    #[constructor(order = 1)]
    status: crate::http_contract_status::HttpContractStatus,
}

impl HttpContractObservation {
    pub(super) const fn parts(
        &self,
    ) -> (
        &crate::http_contract_body::HttpContractBody,
        frontend_contract::route_metadata::RouteMetadata,
        crate::http_contract_status::HttpContractStatus,
    ) {
        (&self.body, self.metadata, self.status)
    }
}
