#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenApiOperationExpectation {
    pub(super) content_type: frontend_contract::contract_str::ContractStr,
    pub(super) metadata: frontend_contract::route_metadata::RouteMetadata,
    pub(super) security: crate::open_api_security_expectation::OpenApiSecurityExpectation,
    pub(super) status: crate::open_api_response_status::OpenApiResponseStatus,
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
}
