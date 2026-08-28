#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::wildcard_imports,
    reason = "root-owned route modules retain generated-registry grouping, utoipa expansion behavior, and the vocabulary previously inherited from the route owner module"
)]

pub use common_routes::common_routes;
mod arc_common_routes_app_state;
mod axum_common_routes;
mod axum_health_check_status;
#[cfg(test)]
#[cfg(test)]
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
pub use crate::arc_common_routes_app_state::*;
pub use crate::axum_common_routes::*;
pub(crate) use crate::axum_health_check_status::*;
#[cfg(test)]
pub(crate) use crate::axum_http_uri_ref::*;
pub use crate::common_no_body::*;
pub(crate) use crate::common_not_found_error::*;
pub use crate::common_route::*;
pub use crate::common_routes_open_api::*;
pub use crate::common_routes_parameters::*;
pub use crate::git_info::*;
pub use crate::git_info_route::*;
pub(crate) use crate::health_check_error::*;
pub use crate::health_check_route::*;
pub(crate) use crate::health_check_succeeded::*;
pub use crate::health_component::*;
pub use crate::health_component_kind::*;
pub use crate::health_components::*;
pub use crate::health_components_error::*;
pub use crate::health_database_available::*;
pub(crate) use crate::health_error::*;
pub use crate::health_live_route::*;
pub(crate) use crate::health_probe_timeout::*;
pub use crate::health_ready_route::*;
pub use crate::health_report::*;
pub use crate::health_route::*;
pub use crate::health_status::*;
pub(crate) use crate::json_res::*;
#[cfg(test)]
pub(crate) use crate::make_git_info_payload::*;
pub(crate) use crate::make_json_response::*;
#[cfg(test)]
pub(crate) use crate::make_not_found_payload::*;
#[cfg(test)]
pub(crate) use crate::map_health_check_status::*;
pub(crate) use crate::not_found_payload::*;
pub use crate::utoipa_common_routes_open_api_document::*;
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
mod json_res;
#[cfg(test)]
mod make_git_info_payload;
mod make_json_response;
#[cfg(test)]
mod make_no_route_message;
#[cfg(test)]
mod make_no_route_message_for_suffix;
#[cfg(test)]
mod make_not_found_payload;
#[cfg(test)]
mod make_not_found_payload_with_message;
#[cfg(test)]
mod map_health_check_status;
#[cfg(test)]
#[cfg(test)]
mod no_route_message_capacity;
mod not_found_payload;
mod open_api_specification_path;
mod readiness_report;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod uri_suffix;
#[cfg(test)]
#[cfg(test)]
mod uri_suffix_ref;
mod utoipa_common_routes_open_api_document;

pub(crate) use axum_json_payload::AxumJsonPayload;
pub(crate) use health_components_max_len::HEALTH_COMPONENTS_MAX_LEN;
#[cfg(test)]
pub(crate) use health_report_response::health_report_response;
#[cfg(test)]
pub(crate) use make_no_route_message::make_no_route_message;
#[cfg(test)]
pub(crate) use make_no_route_message_for_suffix::make_no_route_message_for_suffix;
#[cfg(test)]
pub(crate) use make_not_found_payload_with_message::make_not_found_payload_with_message;
#[cfg(test)]
pub(crate) use no_route_message_capacity::NoRouteMessageCapacity;
pub(crate) use open_api_specification_path::OpenApiSpecificationPath;
#[cfg(test)]
pub(crate) use uri_suffix::uri_suffix;
#[cfg(test)]
pub(crate) use uri_suffix_ref::UriSuffixRef;
