#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "generated route registries stay adjacent to their endpoints and utoipa expands to an internal for_each"
)]
#[path = "git_info.rs"]
mod git_info;
pub use git_info::*;
#[path = "health_database_available.rs"]
mod health_database_available;
pub use health_database_available::*;
#[path = "health_status.rs"]
mod health_status;
pub use health_status::*;
#[path = "health_component_kind.rs"]
mod health_component_kind;
pub use health_component_kind::*;
#[path = "health_component.rs"]
mod health_component;
pub use health_component::*;
#[path = "health_components.rs"]
mod health_components;
pub use health_components::*;
#[path = "health_components_error.rs"]
mod health_components_error;
pub use health_components_error::*;
#[path = "health_report.rs"]
mod health_report;
pub use health_report::*;
#[path = "common_no_body.rs"]
mod common_no_body;
pub use common_no_body::*;
#[path = "health_live_route.rs"]
mod health_live_route;
pub use health_live_route::*;
#[path = "health_ready_route.rs"]
mod health_ready_route;
pub use health_ready_route::*;
#[path = "health_route.rs"]
mod health_route;
pub use health_route::*;
#[path = "health_check_route.rs"]
mod health_check_route;
pub use health_check_route::*;
#[path = "git_info_route.rs"]
mod git_info_route;
pub use git_info_route::*;
#[path = "common_route.rs"]
mod common_route;
pub use common_route::*;
#[path = "axum_common_routes.rs"]
mod axum_common_routes;
pub use axum_common_routes::*;
#[path = "arc_common_routes_app_state.rs"]
mod arc_common_routes_app_state;
pub use arc_common_routes_app_state::*;
#[path = "common_routes_open_api.rs"]
mod common_routes_open_api;
pub use common_routes_open_api::*;
#[path = "utoipa_common_routes_open_api_document.rs"]
mod utoipa_common_routes_open_api_document;
pub use utoipa_common_routes_open_api_document::*;
#[path = "common_routes_parameters.rs"]
mod common_routes_parameters;
pub use common_routes_parameters::*;
#[path = "not_found_payload.rs"]
mod not_found_payload;
pub(crate) use not_found_payload::*;
#[path = "axum_http_uri_ref.rs"]
mod axum_http_uri_ref;
pub(crate) use axum_http_uri_ref::*;
#[path = "health_check_succeeded.rs"]
mod health_check_succeeded;
pub(crate) use health_check_succeeded::*;
#[path = "axum_health_check_status.rs"]
mod axum_health_check_status;
pub(crate) use axum_health_check_status::*;
#[path = "json_res.rs"]
mod json_res;
pub(crate) use json_res::*;
#[path = "common_not_found_error.rs"]
mod common_not_found_error;
pub(crate) use common_not_found_error::*;
#[path = "health_check_error.rs"]
mod health_check_error;
pub(crate) use health_check_error::*;
#[path = "health_error.rs"]
mod health_error;
pub(crate) use health_error::*;
#[path = "health_live_error.rs"]
mod health_live_error;
pub(crate) use health_live_error::*;
#[path = "health_ready_error.rs"]
mod health_ready_error;
pub(crate) use health_ready_error::*;
#[path = "make_git_info_payload.rs"]
mod make_git_info_payload;
pub(crate) use make_git_info_payload::*;
#[path = "make_not_found_payload.rs"]
mod make_not_found_payload;
pub(crate) use make_not_found_payload::*;
#[path = "make_commit_json_response.rs"]
mod make_commit_json_response;
pub(crate) use make_commit_json_response::*;
#[path = "make_json_response.rs"]
mod make_json_response;
pub(crate) use make_json_response::*;
#[path = "map_health_check_status.rs"]
mod map_health_check_status;
pub(crate) use map_health_check_status::*;
#[path = "health_probe_timeout.rs"]
mod health_probe_timeout;
pub(crate) use health_probe_timeout::*;
#[path = "open_api_specification_path.rs"]
mod open_api_specification_path;
use open_api_specification_path::OpenApiSpecificationPath;
#[path = "uri_suffix_ref.rs"]
mod uri_suffix_ref;
use uri_suffix_ref::UriSuffixRef;
#[path = "no_route_message_capacity.rs"]
mod no_route_message_capacity;
use no_route_message_capacity::NoRouteMessageCapacity;
#[path = "axum_json_payload.rs"]
mod axum_json_payload;
use axum_json_payload::AxumJsonPayload;
#[path = "health_unavailable_response.rs"]
mod health_unavailable_response;
use health_unavailable_response::health_unavailable_response;
#[path = "make_no_route_message.rs"]
mod make_no_route_message;
use make_no_route_message::make_no_route_message;
#[path = "make_no_route_message_for_suffix.rs"]
mod make_no_route_message_for_suffix;
use make_no_route_message_for_suffix::make_no_route_message_for_suffix;
#[path = "uri_suffix.rs"]
mod uri_suffix;
use uri_suffix::uri_suffix;
#[path = "make_not_found_payload_with_message.rs"]
mod make_not_found_payload_with_message;
use make_not_found_payload_with_message::make_not_found_payload_with_message;
#[path = "health_components_max_len.rs"]
mod health_components_max_len;
use health_components_max_len::HEALTH_COMPONENTS_MAX_LEN;

#[cfg(test)]
#[path = "domain_types_tests.rs"]
mod tests;
