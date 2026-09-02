#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractObservation {
    body: crate::http_contract_body::HttpContractBody,
    metadata: frontend_contract::route_metadata::RouteMetadata,
    status: crate::http_contract_status::HttpContractStatus,
}

impl HttpContractObservation {
    #[must_use]
    pub const fn new(
        route_metadata: frontend_contract::route_metadata::RouteMetadata,
        http_contract_status: crate::http_contract_status::HttpContractStatus,
        http_contract_body: crate::http_contract_body::HttpContractBody,
    ) -> Self {
        Self {
            body: http_contract_body,
            metadata: route_metadata,
            status: http_contract_status,
        }
    }

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
