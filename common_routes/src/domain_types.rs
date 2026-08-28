#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "generated route registries stay adjacent to their endpoints and utoipa expands to an internal for_each"
)]
pub use crate::arc_common_routes_app_state::*;
pub use crate::axum_common_routes::*;
pub(crate) use crate::axum_health_check_status::*;
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
pub(crate) use crate::make_commit_json_response::*;
pub(crate) use crate::make_git_info_payload::*;
pub(crate) use crate::make_json_response::*;
pub(crate) use crate::make_not_found_payload::*;
pub(crate) use crate::map_health_check_status::*;
pub(crate) use crate::not_found_payload::*;
pub use crate::utoipa_common_routes_open_api_document::*;
