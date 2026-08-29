#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::openapi_validation::{OpenApiResponseStatus, OpenApiSecurityExpectation};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenApiOperationExpectation {
    pub(super) content_type: frontend_contract::ContractStr,
    pub(super) metadata: frontend_contract::RouteMetadata,
    pub(super) security: OpenApiSecurityExpectation,
    pub(super) status: OpenApiResponseStatus,
}
impl OpenApiOperationExpectation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::RouteMetadata,
        status: OpenApiResponseStatus,
        content_type: frontend_contract::ContractStr,
        security: OpenApiSecurityExpectation,
    ) -> Self {
        Self {
            content_type,
            metadata,
            security,
            status,
        }
    }
}
