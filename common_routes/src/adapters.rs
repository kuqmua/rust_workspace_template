#[path = "common_route_registry.rs"]
mod common_route_registry;
#[path = "common_routes.rs"]
mod common_routes;
#[path = "database_is_ready.rs"]
mod database_is_ready;
#[path = "git_info_response.rs"]
mod git_info_response;
#[path = "health.rs"]
mod health;
#[path = "health_check.rs"]
mod health_check;
#[path = "health_live.rs"]
mod health_live;
#[path = "health_ready.rs"]
mod health_ready;
#[path = "health_report_response.rs"]
mod health_report_response;
#[path = "open_api.rs"]
mod open_api;
#[path = "readiness_report.rs"]
mod readiness_report;

pub use common_routes::common_routes;
#[cfg(test)]
use health_report_response::health_report_response;
pub(crate) use open_api::open_api;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
