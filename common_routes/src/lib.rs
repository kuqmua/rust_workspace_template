#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::wildcard_imports,
    reason = "root-owned route modules retain generated-registry grouping, utoipa expansion behavior, and the vocabulary previously inherited from the route owner module"
)]

pub mod arc_common_routes_app_state;
pub mod axum_common_routes;
pub mod axum_health_check_status;
#[cfg(test)]
pub mod axum_http_uri_ref;
pub mod axum_json_payload;
pub mod common_no_body;
pub mod common_not_found_error;
pub mod common_route;
pub mod common_route_registry;
pub mod common_routes;
pub mod common_routes_open_api;
pub mod common_routes_parameters;
pub mod database_is_ready;
pub mod domain_types;
#[cfg(test)]
pub mod domain_types_tests;
#[cfg(test)]
pub mod domain_types_tests_health;
#[cfg(test)]
pub mod domain_types_tests_route_contract;
pub mod git_info;
pub mod git_info_response;
pub mod git_info_route;
pub mod health;
pub mod health_check;
pub mod health_check_error;
pub mod health_check_route;
pub mod health_check_succeeded;
pub mod health_component;
pub mod health_component_kind;
pub mod health_components;
pub mod health_components_error;
pub mod health_components_max_len;
pub mod health_database_available;
pub mod health_error;
pub mod health_live;
pub mod health_live_route;
pub mod health_probe_timeout;
pub mod health_ready;
pub mod health_ready_route;
pub mod health_report;
pub mod health_report_response;
pub mod health_route;
pub mod health_status;
pub mod json_res;
#[cfg(test)]
pub mod make_git_info_payload;
pub mod make_json_response;
#[cfg(test)]
pub mod make_no_route_message;
#[cfg(test)]
pub mod make_no_route_message_for_suffix;
#[cfg(test)]
pub mod make_not_found_payload;
#[cfg(test)]
pub mod make_not_found_payload_with_message;
#[cfg(test)]
pub mod map_health_check_status;
#[cfg(test)]
pub mod no_route_message_capacity;
pub mod not_found_payload;
pub mod open_api_specification_path;
pub mod readiness_report;
#[cfg(test)]
pub mod tests;
#[cfg(test)]
pub mod uri_suffix;
#[cfg(test)]
pub mod uri_suffix_ref;
pub mod utoipa_common_routes_open_api_document;
