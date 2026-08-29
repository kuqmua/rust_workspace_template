#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteTestCapabilities {
    pub(super) database: crate::route_database_usage::RouteDatabaseUsage,
    pub(super) json_body: crate::route_json_body_usage::RouteJsonBodyUsage,
    pub(super) response: crate::route_response_kind::RouteResponseKind,
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
