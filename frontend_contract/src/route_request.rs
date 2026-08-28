use super::TypedRoute;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct RouteRequest<Route>
where
    Route: TypedRoute,
{
    body: Route::Request,
}
impl<Route> RouteRequest<Route>
where
    Route: TypedRoute,
{
    #[must_use]
    pub const fn new(body: Route::Request) -> Self {
        Self { body }
    }
    #[must_use]
    pub const fn body(&self) -> &Route::Request {
        &self.body
    }
}
