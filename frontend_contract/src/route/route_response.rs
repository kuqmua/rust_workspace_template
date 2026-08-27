#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::TypedRoute;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct RouteResponse<Route>
where
    Route: TypedRoute,
{
    pub(super) body: Route::Response,
}
impl<Route> RouteResponse<Route>
where
    Route: TypedRoute,
{
    #[must_use]
    pub const fn body(&self) -> &Route::Response {
        &self.body
    }
}
