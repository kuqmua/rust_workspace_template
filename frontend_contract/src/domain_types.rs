#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "contract implementations keep constructors before accessors and fluent modifiers"
)]

pub use crate::auth_session_keep_alive::{
    AuthSessionInstant, AuthSessionKeepAlive, AuthSessionKeepAliveDecision,
    AuthSessionKeepAliveError, AuthSessionPresence, AuthSessionRefreshIntervalDuration,
    AuthSessionRefreshOutcome,
};
pub use crate::client::TypedClient;
pub use crate::problem::{
    ApiProblem, ApiProblemDetail, ApiProblemError, ApiProblemField, ApiProblemKind,
    ApiProblemRequestId, ApiProblemStatus, ApiProblemViolation,
};
pub use crate::route::{
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
pub use crate::route::{AxumMethodFilter, to_axum_method_filter};
pub use crate::route_coverage::{
    AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, RouteAccess, RouteCoverageDescriptor,
    RouteCoverageError, RouteCoverageEvidence, RouteCoverageObligation, RouteDatabaseUsage,
    RouteJsonBodyUsage, RouteMutation, RouteResponseKind, RouteTestCapabilities,
    RouteTestCategories, RouteTestCategory, missing_required_test_categories,
    required_test_categories, validate_route_coverage,
};
#[cfg(not(target_arch = "wasm32"))]
pub use crate::route_registration_contract::{AxumRouteMethodRouter, route_method_router};
pub use crate::route_registration_contract::{RegisteredRoutePath, RouteRegistrationContract};
pub use crate::url_builder::{
    ApiUrl, ApiUrlBuildError, ApiUrlPathSegmentRef, ApiUrlQueryComponentRef,
};
pub use frontend_contract_macros::{
    ContractStructApi, PageCatalog, RouteCatalog, RouteFamily, TypedRoute, UnitEnumCatalog,
    UnitEnumIndex, api_operation_error, endpoint_registry, route_error, route_openapi,
    route_operation, route_registry,
};

pub use crate::field_contract::*;
pub use crate::http_status::*;
pub use crate::page_transport::*;
pub use crate::route_contract::*;
