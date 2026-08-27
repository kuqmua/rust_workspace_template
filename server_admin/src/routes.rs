#![allow(clippy::module_inception)] // the routes function owns the required same-named module
#[path = "routes/admin_auth_route_registry.rs"]
mod admin_auth_route_registry;
#[path = "routes/open_api.rs"]
mod open_api;
#[path = "routes/routes.rs"]
mod routes;

use admin_auth_route_registry::AdminAuthRouteRegistry;
pub use open_api::open_api;
pub use routes::routes;
