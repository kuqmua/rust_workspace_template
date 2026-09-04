#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "http contract expectation keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(proc_macro_new::New)]
pub struct HttpContractExpectation {
    metadata: frontend_contract::route_metadata::RouteMetadata,
    status: crate::http_contract_status::HttpContractStatus,
    body_kind: crate::http_contract_body_kind::HttpContractBodyKind,
}

impl HttpContractExpectation {
    pub(super) const fn parts(
        &self,
    ) -> (
        frontend_contract::route_metadata::RouteMetadata,
        crate::http_contract_status::HttpContractStatus,
        crate::http_contract_body_kind::HttpContractBodyKind,
    ) {
        (self.metadata, self.status, self.body_kind)
    }
}
