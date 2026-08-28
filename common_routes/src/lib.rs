#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::wildcard_imports,
    reason = "root-owned route modules retain generated-registry grouping, utoipa expansion behavior, and the vocabulary previously inherited from the route owner module"
)]

pub mod adapters;
mod arc_common_routes_app_state;
mod axum_common_routes;
mod axum_health_check_status;
mod axum_http_uri_ref;
mod axum_json_payload;
mod common_no_body;
mod common_not_found_error;
mod common_route;
mod common_route_registry;
mod common_routes;
mod common_routes_open_api;
mod common_routes_parameters;
mod database_is_ready;
pub mod domain_types;
#[cfg(test)]
mod domain_types_tests;
#[cfg(test)]
mod domain_types_tests_health;
#[cfg(test)]
mod domain_types_tests_route_contract;
mod git_info;
mod git_info_response;
mod git_info_route;
mod health;
mod health_check;
mod health_check_error;
mod health_check_route;
mod health_check_succeeded;
mod health_component;
mod health_component_kind;
mod health_components;
mod health_components_error;
mod health_components_max_len;
mod health_database_available;
mod health_error;
mod health_live;
mod health_live_route;
mod health_probe_timeout;
mod health_ready;
mod health_ready_route;
mod health_report;
mod health_report_response;
mod health_route;
mod health_status;
mod health_unavailable_response;
mod json_res;
mod make_commit_json_response;
mod make_git_info_payload;
mod make_json_response;
mod make_no_route_message;
mod make_no_route_message_for_suffix;
mod make_not_found_payload;
mod make_not_found_payload_with_message;
mod map_health_check_status;
mod no_route_message_capacity;
mod not_found_payload;
mod open_api;
mod open_api_specification_path;
mod readiness_report;
#[cfg(test)]
mod tests;
mod uri_suffix;
mod uri_suffix_ref;
mod utoipa_common_routes_open_api_document;

pub(crate) use axum_json_payload::AxumJsonPayload;
pub(crate) use domain_types::*;
pub(crate) use health_components_max_len::HEALTH_COMPONENTS_MAX_LEN;
#[cfg(test)]
pub(crate) use health_report_response::health_report_response;
pub(crate) use health_unavailable_response::health_unavailable_response;
pub(crate) use make_no_route_message::make_no_route_message;
pub(crate) use make_no_route_message_for_suffix::make_no_route_message_for_suffix;
pub(crate) use make_not_found_payload_with_message::make_not_found_payload_with_message;
pub(crate) use no_route_message_capacity::NoRouteMessageCapacity;
pub(crate) use open_api_specification_path::OpenApiSpecificationPath;
pub(crate) use uri_suffix::uri_suffix;
pub(crate) use uri_suffix_ref::UriSuffixRef;
