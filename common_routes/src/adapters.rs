#[path = "adapters/common_route_registry.rs"]
mod common_route_registry;
#[path = "adapters/common_routes.rs"]
mod common_routes;
#[path = "adapters/database_is_ready.rs"]
mod database_is_ready;
#[path = "adapters/git_info.rs"]
mod git_info;
#[path = "adapters/health.rs"]
mod health;
#[path = "adapters/health_check.rs"]
mod health_check;
#[path = "adapters/health_live.rs"]
mod health_live;
#[path = "adapters/health_ready.rs"]
mod health_ready;
#[path = "adapters/health_report_response.rs"]
mod health_report_response;
#[path = "adapters/open_api.rs"]
mod open_api;
#[path = "adapters/readiness_report.rs"]
mod readiness_report;

pub use common_routes::common_routes;
#[cfg(test)]
use health_report_response::health_report_response;
pub(crate) use open_api::open_api;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
