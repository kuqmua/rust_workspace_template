#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
)]
pub struct RouteTestCapabilities {
    database: crate::route_database_usage::RouteDatabaseUsage,
    json_body: crate::route_json_body_usage::RouteJsonBodyUsage,
    response: crate::route_response_kind::RouteResponseKind,
}

impl RouteTestCapabilities {
    #[must_use]
    pub const fn new(
        database: crate::route_database_usage::RouteDatabaseUsage,
        json_body: crate::route_json_body_usage::RouteJsonBodyUsage,
        response: crate::route_response_kind::RouteResponseKind,
    ) -> Self {
        Self {
            database,
            json_body,
            response,
        }
    }
}
