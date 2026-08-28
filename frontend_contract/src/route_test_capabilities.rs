#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{RouteDatabaseUsage, RouteJsonBodyUsage, RouteResponseKind};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteTestCapabilities {
    pub(super) database: RouteDatabaseUsage,
    pub(super) json_body: RouteJsonBodyUsage,
    pub(super) response: RouteResponseKind,
}

impl RouteTestCapabilities {
    #[must_use]
    pub const fn new(
        database: RouteDatabaseUsage,
        json_body: RouteJsonBodyUsage,
        response: RouteResponseKind,
    ) -> Self {
        Self {
            database,
            json_body,
            response,
        }
    }
}
