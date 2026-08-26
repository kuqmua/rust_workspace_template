#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "contract implementations keep constructors before accessors and fluent modifiers"
)]

#[path = "auth_session_keep_alive.rs"]
mod auth_session_keep_alive;
#[path = "client.rs"]
mod client;
#[path = "problem.rs"]
mod problem;
#[path = "route.rs"]
mod route;
#[path = "route_coverage.rs"]
mod route_coverage;
#[path = "route_registration_contract.rs"]
mod route_registration_contract;
#[path = "url_builder.rs"]
mod url_builder;
pub use auth_session_keep_alive::{
    AuthSessionInstant, AuthSessionKeepAlive, AuthSessionKeepAliveDecision,
    AuthSessionKeepAliveError, AuthSessionPresence, AuthSessionRefreshIntervalDuration,
    AuthSessionRefreshOutcome,
};
pub use client::TypedClient;
pub use frontend_contract_macros::{
    ContractStructApi, PageCatalog, RouteCatalog, RouteFamily, TypedRoute, UnitEnumCatalog,
    UnitEnumIndex, api_operation_error, endpoint_registry, route_error, route_openapi,
    route_operation, route_registry,
};
pub use problem::{
    ApiProblem, ApiProblemDetail, ApiProblemError, ApiProblemField, ApiProblemKind,
    ApiProblemRequestId, ApiProblemStatus, ApiProblemViolation,
};
pub use route::{
    AuthenticatedTransport, CoveredRoute, OpenApiSecuritySchemeRef, ParameterizedRoute,
    ParameterizedRoutePath, ParameterizedRoutePathTryFromStringError, PublicTransport,
    RouteBodyLimit, RouteCoverageDescriptors, RouteFamily, RouteInFamily, RouteMetadata,
    RouteMetadataList, RouteMethod, RouteRequest, RouteRequestBody, RouteResponse,
    RouteSchemaContract, RouteSchemaContracts, RouteTransport, TypedRoute,
    UtoipaOpenApiComponentsRefMut, UtoipaOpenApiPathParameter, UtoipaOpenApiRefMut,
    UtoipaOpenApiRouteSchema, apply_openapi_error_contract, apply_openapi_path_parameter_contract,
    apply_openapi_request_contract, apply_openapi_security_contract,
    apply_openapi_success_contract, client_request, client_route_metadata, openapi_route_metadata,
    register_openapi_route_schemas, register_openapi_schema, server_response,
    server_route_metadata, typed_parameterized_route_path, typed_route_path,
};
#[cfg(not(target_arch = "wasm32"))]
pub use route::{AxumMethodFilter, axum_method_filter};
pub use route_coverage::{
    AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, RouteAccess, RouteCoverageDescriptor,
    RouteCoverageError, RouteCoverageEvidence, RouteCoverageObligation, RouteDatabaseUsage,
    RouteJsonBodyUsage, RouteMutation, RouteResponseKind, RouteTestCapabilities,
    RouteTestCategories, RouteTestCategory, missing_required_test_categories,
    required_test_categories, validate_route_coverage,
};
#[cfg(not(target_arch = "wasm32"))]
pub use route_registration_contract::{AxumRouteMethodRouter, route_method_router};
pub use route_registration_contract::{RegisteredRoutePath, RouteRegistrationContract};
pub use url_builder::{ApiUrl, ApiUrlBuildError, ApiUrlPathSegmentRef, ApiUrlQueryComponentRef};
#[path = "field_contract.rs"]
mod field_contract;
#[path = "http_status.rs"]
mod http_status;
#[path = "page_transport.rs"]
mod page_transport;
#[path = "route_contract.rs"]
mod route_contract;

pub use field_contract::*;
pub use http_status::*;
pub use page_transport::*;
pub use route_contract::*;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
