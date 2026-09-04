#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
#[getters(bare)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "http contract expectation keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(proc_macro_new::New)]
pub struct HttpContractExpectation {
    #[getters(copy)]
    metadata: frontend_contract::route_metadata::RouteMetadata,
    #[getters(copy)]
    status: crate::http_contract_status::HttpContractStatus,
    #[getters(copy)]
    body_kind: crate::http_contract_body_kind::HttpContractBodyKind,
}
