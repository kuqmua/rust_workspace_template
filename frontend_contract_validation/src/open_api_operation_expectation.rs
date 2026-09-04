#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct OpenApiOperationExpectation {
    #[constructor(order = 2)]
    content_type: frontend_contract::contract_str::ContractStr,
    #[constructor(order = 0)]
    metadata: frontend_contract::route_metadata::RouteMetadata,
    #[constructor(order = 3)]
    security: crate::open_api_security_expectation::OpenApiSecurityExpectation,
    #[constructor(order = 1)]
    status: crate::open_api_response_status::OpenApiResponseStatus,
}
impl OpenApiOperationExpectation {
    pub(super) const fn parts(
        &self,
    ) -> (
        frontend_contract::contract_str::ContractStr,
        frontend_contract::route_metadata::RouteMetadata,
        crate::open_api_security_expectation::OpenApiSecurityExpectation,
        crate::open_api_response_status::OpenApiResponseStatus,
    ) {
        (self.content_type, self.metadata, self.security, self.status)
    }
}
