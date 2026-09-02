#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
pub struct RouteTestCapabilities {
    database: crate::route_database_usage::RouteDatabaseUsage,
    json_body: crate::route_json_body_usage::RouteJsonBodyUsage,
    response: crate::route_response_kind::RouteResponseKind,
}

impl RouteTestCapabilities {
    #[must_use]
    pub const fn new(
        route_database_usage: crate::route_database_usage::RouteDatabaseUsage,
        route_json_body_usage: crate::route_json_body_usage::RouteJsonBodyUsage,
        route_response_kind: crate::route_response_kind::RouteResponseKind,
    ) -> Self {
        Self {
            database: route_database_usage,
            json_body: route_json_body_usage,
            response: route_response_kind,
        }
    }
}
