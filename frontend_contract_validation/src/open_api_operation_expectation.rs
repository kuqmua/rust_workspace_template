#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenApiOperationExpectation {
    content_type: frontend_contract::contract_str::ContractStr,
    metadata: frontend_contract::route_metadata::RouteMetadata,
    security: crate::open_api_security_expectation::OpenApiSecurityExpectation,
    status: crate::open_api_response_status::OpenApiResponseStatus,
}
impl OpenApiOperationExpectation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::route_metadata::RouteMetadata,
        status: crate::open_api_response_status::OpenApiResponseStatus,
        content_type: frontend_contract::contract_str::ContractStr,
        security: crate::open_api_security_expectation::OpenApiSecurityExpectation,
    ) -> Self {
        Self {
            content_type,
            metadata,
            security,
            status,
        }
    }

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
