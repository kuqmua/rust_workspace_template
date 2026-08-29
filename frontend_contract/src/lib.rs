#![allow(
    unused_imports,
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "root-owned modules retain the vocabulary and constructor-before-accessor ordering previously inherited from the contract owner module"
)]

mod action_contract;
pub(crate) use action_contract::*;
mod action_contracts;
pub(crate) use action_contracts::*;
mod api_problem;
pub(crate) use api_problem::*;
mod api_problem_detail;
pub(crate) use api_problem_detail::*;
mod api_problem_error;
pub(crate) use api_problem_error::*;
mod api_problem_field;
pub(crate) use api_problem_field::*;
mod api_problem_kind;
pub(crate) use api_problem_kind::*;
mod api_problem_request_id;
pub(crate) use api_problem_request_id::*;
mod api_problem_status;
pub(crate) use api_problem_status::*;
mod api_problem_violation;
pub(crate) use api_problem_violation::*;
mod api_problem_violations;
pub(crate) use api_problem_violations::*;
mod api_url;
pub(crate) use api_url::*;
mod api_url_build_error;
pub(crate) use api_url_build_error::*;
mod api_url_component_encode_set;
pub(crate) use api_url_component_encode_set::*;
mod api_url_path_segment_ref;
pub(crate) use api_url_path_segment_ref::*;
mod api_url_query_component_ref;
pub(crate) use api_url_query_component_ref::*;
#[path = "apply_openapi_error_contract.rs"]
mod apply_openapi_error_contract_fn;
pub(crate) use apply_openapi_error_contract_fn::*;
#[path = "apply_openapi_path_parameter_contract.rs"]
mod apply_openapi_path_parameter_contract_fn;
pub(crate) use apply_openapi_path_parameter_contract_fn::*;
#[path = "apply_openapi_request_contract.rs"]
mod apply_openapi_request_contract_fn;
pub(crate) use apply_openapi_request_contract_fn::*;
#[path = "apply_openapi_security_contract.rs"]
mod apply_openapi_security_contract_fn;
pub(crate) use apply_openapi_security_contract_fn::*;
#[path = "apply_openapi_success_contract.rs"]
mod apply_openapi_success_contract_fn;
pub(crate) use apply_openapi_success_contract_fn::*;
mod auth_session_instant;
pub(crate) use auth_session_instant::*;
mod auth_session_keep_alive;
pub(crate) use auth_session_keep_alive::*;
mod auth_session_keep_alive_decision;
pub(crate) use auth_session_keep_alive_decision::*;
mod auth_session_keep_alive_error;
pub(crate) use auth_session_keep_alive_error::*;
mod auth_session_presence;
pub(crate) use auth_session_presence::*;
mod auth_session_refresh_interval_duration;
pub(crate) use auth_session_refresh_interval_duration::*;
mod auth_session_refresh_outcome;
pub(crate) use auth_session_refresh_outcome::*;
mod auth_session_refresh_state;
pub(crate) use auth_session_refresh_state::*;
mod authenticated_transport;
pub(crate) use authenticated_transport::*;
mod authentication_requirement;
pub(crate) use authentication_requirement::*;
mod axum_method_filter;
pub(crate) use axum_method_filter::*;
mod axum_route_method_router;
pub(crate) use axum_route_method_router::*;
mod capability_support;
pub(crate) use capability_support::*;
mod client;
pub(crate) use client::*;
mod client_error;
pub(crate) use client_error::*;
#[path = "client_request.rs"]
mod client_request_fn;
pub(crate) use client_request_fn::*;
#[path = "client_route_metadata.rs"]
mod client_route_metadata_fn;
pub(crate) use client_route_metadata_fn::*;
mod confirmation_requirement;
pub(crate) use confirmation_requirement::*;
mod contract_i64;
pub(crate) use contract_i64::*;
mod contract_str;
pub(crate) use contract_str::*;
mod covered_route;
pub(crate) use covered_route::*;
mod create_form_value_error;
pub(crate) use create_form_value_error::*;
mod decode_api_problem;
pub(crate) use decode_api_problem::*;
mod empty_filter_contracts;
pub(crate) use empty_filter_contracts::*;
mod field_capability;
pub(crate) use field_capability::*;
pub mod field_contract;
mod field_contracts;
pub(crate) use field_contracts::*;
mod field_order;
pub(crate) use field_order::*;
mod field_placeholder;
pub(crate) use field_placeholder::*;
mod field_visibility;
pub(crate) use field_visibility::*;
mod filter_contracts;
pub(crate) use filter_contracts::*;
mod filter_form_value_contract;
pub(crate) use filter_form_value_contract::*;
mod filter_operation;
pub(crate) use filter_operation::*;
mod filter_value_shape;
pub(crate) use filter_value_shape::*;
mod filter_wire_json;
pub(crate) use filter_wire_json::*;
mod form_field_error;
pub(crate) use form_field_error::*;
mod form_field_name_ref;
pub(crate) use form_field_name_ref::*;
mod form_value;
pub(crate) use form_value::*;
mod form_value_contract;
pub(crate) use form_value_contract::*;
mod form_value_error;
pub(crate) use form_value_error::*;
mod form_value_ref;
pub(crate) use form_value_ref::*;
mod frontend_contract_body_error;
pub(crate) use frontend_contract_body_error::*;
mod has_filter_contracts;
pub(crate) use has_filter_contracts::*;
mod has_type_contract;
pub(crate) use has_type_contract::*;
mod http_status_try_from_u16_error;
pub(crate) use http_status_try_from_u16_error::*;
mod input_kind;
pub(crate) use input_kind::*;
mod input_step;
pub(crate) use input_step::*;
mod known_http_status;
pub(crate) use known_http_status::*;
mod missing_required_test_categories;
pub(crate) use missing_required_test_categories::*;
mod mutation_kind;
pub(crate) use mutation_kind::*;
mod nullability;
pub(crate) use nullability::*;
mod numeric_bound;
pub(crate) use numeric_bound::*;
mod open_api_security_scheme_ref;
pub(crate) use open_api_security_scheme_ref::*;
#[path = "openapi_route_metadata.rs"]
mod openapi_route_metadata_fn;
pub(crate) use openapi_route_metadata_fn::*;
mod operation_kind;
pub(crate) use operation_kind::*;
mod page_contract;
pub(crate) use page_contract::*;
pub mod page_transport;
mod parameterized_route;
pub(crate) use parameterized_route::*;
mod parameterized_route_path;
pub(crate) use parameterized_route_path::*;
mod parameterized_route_path_try_from_string_error;
pub(crate) use parameterized_route_path_try_from_string_error::*;
mod primary_key_kind;
pub(crate) use primary_key_kind::*;
mod problem;
pub(crate) use problem::*;
mod public_transport;
pub(crate) use public_transport::*;
#[path = "register_openapi_route_schemas.rs"]
mod register_openapi_route_schemas_fn;
pub(crate) use register_openapi_route_schemas_fn::*;
#[path = "register_openapi_schema.rs"]
mod register_openapi_schema_fn;
pub(crate) use register_openapi_schema_fn::*;
mod registered_route_path;
pub(crate) use registered_route_path::*;
mod required_test_categories;
pub(crate) use required_test_categories::*;
mod route;
pub(crate) use route::*;
mod route_access;
pub(crate) use route_access::*;
mod route_body_limit;
pub(crate) use route_body_limit::*;
pub mod route_contract;
mod route_contracts;
pub(crate) use route_contracts::*;
mod route_coverage;
pub(crate) use route_coverage::*;
mod route_coverage_descriptor;
pub(crate) use route_coverage_descriptor::*;
mod route_coverage_descriptors;
pub(crate) use route_coverage_descriptors::*;
mod route_coverage_error;
pub(crate) use route_coverage_error::*;
mod route_coverage_evidence;
pub(crate) use route_coverage_evidence::*;
mod route_coverage_obligation;
pub(crate) use route_coverage_obligation::*;
mod route_database_usage;
pub(crate) use route_database_usage::*;
mod route_error_policy;
pub(crate) use route_error_policy::*;
mod route_error_status;
pub(crate) use route_error_status::*;
mod route_family;
pub use route_family::*;
mod route_in_family;
pub(crate) use route_in_family::*;
mod route_json_body_usage;
pub(crate) use route_json_body_usage::*;
mod route_metadata;
pub(crate) use route_metadata::*;
mod route_metadata_list;
pub(crate) use route_metadata_list::*;
mod route_method;
pub(crate) use route_method::*;
mod route_method_router;
pub(crate) use route_method_router::*;
mod route_mutation;
pub(crate) use route_mutation::*;
mod route_registration_contract;
pub(crate) use route_registration_contract::*;
mod route_request;
pub(crate) use route_request::*;
mod route_request_body;
pub(crate) use route_request_body::*;
mod route_response;
pub(crate) use route_response::*;
mod route_response_kind;
pub(crate) use route_response_kind::*;
mod route_schema_contract;
pub(crate) use route_schema_contract::*;
mod route_schema_contracts;
pub(crate) use route_schema_contracts::*;
mod route_test_capabilities;
pub(crate) use route_test_capabilities::*;
mod route_test_categories;
pub(crate) use route_test_categories::*;
mod route_test_category;
pub(crate) use route_test_category::*;
mod route_transport;
pub(crate) use route_transport::*;
#[path = "server_response.rs"]
mod server_response_fn;
pub(crate) use server_response_fn::*;
#[path = "server_route_metadata.rs"]
mod server_route_metadata_fn;
pub(crate) use server_route_metadata_fn::*;
mod success_status;
pub(crate) use success_status::*;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use tests::*;
#[path = "to_axum_method_filter.rs"]
mod to_axum_method_filter_fn;
pub(crate) use to_axum_method_filter_fn::*;
mod transport;
pub(crate) use transport::*;
mod transport_body;
pub(crate) use transport_body::*;
mod transport_error;
pub(crate) use transport_error::*;
mod transport_idempotency_key;
pub(crate) use transport_idempotency_key::*;
mod transport_if_match;
pub(crate) use transport_if_match::*;
mod transport_path;
pub(crate) use transport_path::*;
mod transport_request;
pub(crate) use transport_request::*;
mod transport_response;
pub(crate) use transport_response::*;
mod transport_retry_after;
pub(crate) use transport_retry_after::*;
mod transport_status;
pub(crate) use transport_status::*;
mod type_contract;
pub(crate) use type_contract::*;
mod typed_client;
pub(crate) use typed_client::*;
#[path = "typed_parameterized_route_path.rs"]
mod typed_parameterized_route_path_fn;
pub(crate) use typed_parameterized_route_path_fn::*;
mod typed_route;
pub use typed_route::*;
#[path = "typed_route_path.rs"]
mod typed_route_path_fn;
pub(crate) use typed_route_path_fn::*;
mod url_builder;
pub(crate) use url_builder::*;
mod utoipa_open_api_components_ref_mut;
pub(crate) use utoipa_open_api_components_ref_mut::*;
mod utoipa_open_api_path_parameter;
pub(crate) use utoipa_open_api_path_parameter::*;
mod utoipa_open_api_ref_mut;
pub(crate) use utoipa_open_api_ref_mut::*;
mod utoipa_open_api_route_schema;
pub(crate) use utoipa_open_api_route_schema::*;
mod validate_route_coverage;
pub(crate) use validate_route_coverage::*;
mod value_example;
pub(crate) use value_example::*;
mod value_format;
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
    RouteBodyLimit, RouteCoverageDescriptors, RouteInFamily, RouteMetadata, RouteMetadataList,
    RouteMethod, RouteRequest, RouteRequestBody, RouteResponse, RouteSchemaContract,
    RouteSchemaContracts, RouteTransport, UtoipaOpenApiComponentsRefMut,
    UtoipaOpenApiPathParameter, UtoipaOpenApiRefMut, UtoipaOpenApiRouteSchema,
    apply_openapi_error_contract, apply_openapi_path_parameter_contract,
    apply_openapi_request_contract, apply_openapi_security_contract,
    apply_openapi_success_contract, client_request, client_route_metadata, openapi_route_metadata,
    register_openapi_route_schemas, register_openapi_schema, server_response,
    server_route_metadata, typed_parameterized_route_path, typed_route_path,
};
#[cfg(not(target_arch = "wasm32"))]
pub use route::{AxumMethodFilter, to_axum_method_filter};
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
pub(crate) use value_format::*;

pub use field_contract::*;
pub use frontend_contract_body_error::FrontendContractBodyError;
pub use http_status_try_from_u16_error::HttpStatusTryFromU16Error;
pub use known_http_status::KnownHttpStatus;
pub use page_transport::*;
pub use route_contract::*;
